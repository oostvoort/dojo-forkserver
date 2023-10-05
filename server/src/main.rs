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
use log::debug;
use tokio::task;
use tokio::time::sleep;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::cors::{Any, CorsLayer};
use server::{CommandManager, get_env, is_port_open, run_sozo};
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
    let current_directory = current_dir().unwrap().to_str().unwrap().to_string();
    let contracts_dir = format!("{}/contracts", current_directory.clone());
    let rpc_url = format!("http://localhost:{}", katana_port.clone());
    let world_address_inner: String;

    loop {
        if is_port_open(katana_port.clone().parse().unwrap()) {
            sleep(Duration::from_secs(1)).await
        } else {
            let accounts = get_accounts(json_rpc_client).await;
            let master = accounts.first().unwrap().clone();

            world_address_inner = run_sozo(katana_port.clone(), format!("{}/Scarb.toml", contracts_dir.clone()), &master.private_key, &master.address);

            let mut world_address_lock = world_address.lock().unwrap();
            *world_address_lock = world_address_inner.clone();

            Command::new("scarb")
                .args([
                    "--manifest-path",
                    &format!("{}/Scarb.toml", contracts_dir),
                    "run",
                    "post_deploy",
                    &world_address_inner.clone(),
                    &master.private_key,
                    &master.address,
                    &rpc_url.clone()
                ])
                .spawn()
                .expect("Default authorizations set");

            break
        }
    };

    let torii = CommandManager::new(
        "torii",
        Some(format!("\
                --rpc {} \
                --database-url sqlite:///{}/database/indexer.db \
                -w {} \
                --manifest {}/target/dev/manifest.json",
                     rpc_url,
                     current_directory,
                     world_address_inner,
                     contracts_dir
        ))
    );

    torii.start_command().await
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
            katana_port.clone(),
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
    // TODO will also need to make sure that katana and torii stops
    katana.abort();
    deploy_contracts.abort();
}