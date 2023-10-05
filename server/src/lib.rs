use std::env;
use std::net::TcpListener;
use std::process::{Child, Command, Stdio};
use log::{error, info};
use tokio::sync::RwLock;
use regex::Regex;

pub struct CommandManager {
    command: RwLock<Command>,
    args: RwLock<Option<String>>,
    handler: RwLock<Option<Child>>
}

impl CommandManager {
    pub fn new(command: &str, args: Option<String>) -> Self {
        Self {
            command: RwLock::new(Command::new(command)),
            args: RwLock::new(args),
            handler: RwLock::new(None)
        }
    }

    pub async fn start_command(&self) {
        let mut command = self.command.write().await;
        if let Some(args) = self.args.read().await.clone() {
            let args = args.split(" ");
            command.args(args);
        };

        let mut handler = self.handler.write().await;
        let spawned_command = command.spawn();
        match spawned_command {
            Ok(spawned_command) => *handler = Some(spawned_command),
            Err(e) => {
                error!("Could not spawn command {}", e.to_string())
            }
        }


    }

    pub async fn reset_command(&self, new_command: Command, new_args: Option<String>) {
        let mut command = self.command.write().await;
        *command = new_command;

        let mut args = self.args.write().await;
        *args = new_args;

        let mut handler = self.handler.write().await;
        *handler = None;
    }

    pub async fn stop_command(&self) {
        let mut handler = self.handler.write().await;
        let kill_handler = handler.take();
        if let Some(mut kill_handler) = kill_handler {
            kill_handler.kill().expect("killed handler");
        }
    }
}

pub fn get_env(key: &str, default: &str) -> String {
    // Check .env file
    match dotenvy::var(key) {
        Ok(v) => return v,
        Err(_) => {
            // Check linux env
            match env::var(key) {
                Ok(v) => return v,
                Err(_) => {
                    // def
                    info!("{key} not provided, defaulting to {default}");
                    default.to_string()
                }
            }
        }
    }
}

pub fn is_port_open(port: u16) -> bool {
    if let Ok(listener) = TcpListener::bind(format!("127.0.0.1:{}", port)) {
        // The port is available, so close the listener and return true
        drop(listener);
        true
    } else {
        // The port is still occupied
        false
    }
}

pub fn run_sozo(
    katana_port: String,
    manifest_path: String,
    private_key: &String,
    account_address: &String
) -> String {

    let output = Command::new("sozo")
        .args([
            "migrate",
            "--rpc-url",
            &format!("http://localhost:{}", katana_port),
            "--manifest-path",
            &manifest_path,
            "--private-key",
            private_key,
            "--account-address",
            account_address
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output().unwrap();

    // Capture the stdout and stderr as strings
    let stdout = String::from_utf8_lossy(&output.stdout);

    let mut starting_index = 0_usize;

    let to_look_for = "at address ";

    if let Some(index) = stdout.find(to_look_for) {
        starting_index = index;
    }

    let world_address = &stdout[starting_index + to_look_for.len()..];

    // Create a regex pattern to match one or more whitespace characters
    let re = Regex::new(r"\s+").unwrap();

    // Replace multiple whitespace characters with a single space
    re.replace_all(world_address, "").to_string()
}