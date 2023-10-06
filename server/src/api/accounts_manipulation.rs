use axum::{Extension, response::Json };
use axum::response::IntoResponse;
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::params::ArrayParams;
use jsonrpsee_http_client::{HttpClient};
use serde::{Deserialize, Serialize};
use starknet::core::types::FieldElement;
use serde_json::json;
use crate::ServerState;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Account {
    pub balance: FieldElement,
    pub public_key: String,
    pub private_key: String,
    pub address: String,
    pub class_hash: String
}

pub async fn get_accounts(client: &HttpClient) -> Vec<Account> {
    client.request::<Vec<Account>, ArrayParams>(
        "katana_predeployedAccounts",
        ArrayParams::default()
    ).await.unwrap_or(Vec::new())
}

pub async fn handler(Extension(state): Extension<ServerState>) -> impl IntoResponse {
    let json_rpc_client = &state.json_rpc_client;

    let accounts = get_accounts(json_rpc_client).await;
    Json(json!(accounts))
}
