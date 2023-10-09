use axum::{Extension, Json};
use axum::response::IntoResponse;
use jsonrpsee_http_client::HttpClient;
use serde::Deserialize;
use crate::ServerState;

#[derive(Deserialize)]
pub struct FundAddress {
    address: String,
    amount: f32,
    token_address: Option<String>,
}

async fn increase_token(
    _client: &HttpClient,
    _address: String,
    _token_address:
    String,
    _amount: f32,
) -> () {
    todo!()
}

async fn increase_eth(
    _client: &HttpClient,
    _address: String,
    _amount: f32
) -> () {
    todo!()
}

pub async fn handler(
    Extension(state): Extension<ServerState>,
    Json(payload): Json<FundAddress>,
) -> impl IntoResponse {
    let json_rpc_client = &state.json_rpc_client;

    match payload.token_address {
        None => increase_eth(
            json_rpc_client,
            payload.address,
            payload.amount,
        ).await,
        Some(token_address) => increase_token(
            json_rpc_client,
            payload.address,
            token_address,
            payload.amount,
        ).await
    }
}