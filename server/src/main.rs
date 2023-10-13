mod api;

use std::convert::Infallible;
use std::env::current_dir;
use std::net::SocketAddr;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use axum::Router;
use axum::http::Method;
use axum::response::Response;
use axum::routing::{get, on, get_service, MethodFilter};
use jsonrpsee_http_client::{HttpClient, HttpClientBuilder};
use log::{debug, error};
use serde::de::Error;
use tokio::task;
use tokio::time::sleep;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::cors::{Any, CorsLayer};
use server::{CommandManager, extract_contract_args, get_env, is_port_open, run_sozo};
use crate::api::accounts_manipulation::get_accounts;

#[derive(Clone)]
pub struct ServerState {
    pub json_rpc_client: HttpClient
}

async fn run_command_manager(manager: CommandManager) {
    manager.start_command().await;
}

async fn run_deploy_contracts(
    json_rpc_client: HttpClient,
    katana_port: String,
    world_address: Arc<Mutex<String>>
) {
    let current_directory = current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let (manifest_json, scarb_dir, rpc_url) =
        (format!("{}/contracts/target/dev/manifest.json", &current_directory),
         format!("{}/contracts/Scarb.toml", &current_directory),
         format!("http://localhost:{}", &katana_port));

    loop {
        if is_port_open(katana_port.parse().unwrap()) {
            sleep(Duration::from_secs(1)).await;
        } else {
            match get_accounts(&json_rpc_client).await.first() {
                Some(master) => {
                    let world_address_inner = match run_sozo(&katana_port, &manifest_json, &scarb_dir, &master.private_key, &master.address){
                        Ok(address) => address,
                        Err(e) => {
                            println!("Could not migrate contracts: {}", e.to_string());
                            error!("Could not migrate contracts: {}", e.to_string());

                            String::new()
                        }
                    };

                    if world_address_inner.is_empty() {
                        break
                    }

                    if let Ok(mut world_address_lock) = world_address.lock() {
                        *world_address_lock = world_address_inner.clone();
                    }

                    let systems = extract_contract_args(&manifest_json);

                    let mut base_args = vec![
                        "--manifest-path".to_string(),
                        scarb_dir.clone(),
                        "run".to_string(),
                        "post_deploy".to_string(),
                        format!("WORLD_ADDRESS={}", &world_address_inner),
                        format!("PRIVATE_KEY={}", &master.private_key),
                        format!("ACCOUNT_ADDRESS={}", &master.address),
                        format!("RPC_URL={}", &rpc_url),
                    ];

                    if let Ok(systems) = systems {
                        base_args.extend(systems);
                    }

                    Command::new("scarb")
                        .args(base_args)
                        .spawn()
                        .expect("Default authorizations set");

                    let torii = CommandManager::new(
                        "torii",
                        Some(format!("\
                            --rpc {} \
                            --database-url sqlite:///{}/database/indexer.db \
                            -w {}",
                                     rpc_url,
                                     current_directory,
                                     world_address_inner
                        ))
                    );

                    torii.start_command().await;
                },
                None => error!("Account not found"),
            }

            break;
        }
    }
}



#[tokio::main]
async fn main() {
    let katana_port = get_env("KATANA_PORT", "5050");

    let server_port = get_env("SERVER_PORT", "3000");
    let server_port: u16 =  server_port.parse().unwrap();

    let world_address = Arc::new(Mutex::new(String::new()));

    let katana = CommandManager::new(
        "katana",
        Some(format!("-p {katana_port}")));
    let katana = task::spawn(run_command_manager(katana));

    // Build json rpc client
    let json_rpc_client = HttpClientBuilder::default()
        .build(format!("http://localhost:{katana_port}"))
        .unwrap();

    let deploy_contracts = task::spawn(
        run_deploy_contracts(
            json_rpc_client.clone(),
            katana_port,
            Arc::clone(&world_address))
    );

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        // allow requests from any origin
        .allow_origin(Any)
        .allow_headers(Any);

    let router = Router::new()
        .route(
            "/api/state",
            get(api::state_management::save_state)
                .on(MethodFilter::PUT, api::state_management::load_state)
                .on(MethodFilter::DELETE, api::state_management::reset_state)
        )
        .route("/api/accounts", get(api::accounts_manipulation::handler))
        .route("/api/world-address", get(move || {
            let world_address_lock = world_address.lock().unwrap();
            let world_address_clone = world_address_lock.clone();
            async move {
                Ok::<_, Infallible>(Response::new(world_address_clone))
            }
        }))
        .route("/api/fund", get(api::funds_manipulation::handler))
        .route("/api/block", on(MethodFilter::POST, api::block_manipulation::handler))
        .nest_service("/fork/assets", get_service(ServeDir::new("./static/fork/assets")))
        .nest_service("/fork", get_service(ServeFile::new("./static/fork/index.html")))
        .nest_service("/assets", get_service(ServeDir::new("./static/assets")))
        .fallback_service(get_service(ServeFile::new("./static/index.html")))
        .layer(cors)
        .layer(AddExtensionLayer::new(ServerState {
            json_rpc_client
        }));

    let addr = SocketAddr::from(([0, 0, 0, 0], server_port));
    debug!("Server started on http://0.0.0.0${server_port}");

    axum::Server::bind(&addr)
        .serve(router.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();

    // Cancel the command manager task when the server stops
    katana.abort();
    deploy_contracts.abort();
}