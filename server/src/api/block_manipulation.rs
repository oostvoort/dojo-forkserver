use axum::{Extension, Json, response::Json as AxumJson};
use axum::response::IntoResponse;
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::params::ArrayParams;
use jsonrpsee_http_client::{HttpClient};
use serde::Deserialize;
use crate::ServerState;

#[derive(Deserialize)]
enum Type {
    MineBlock(u64),
    IncreaseTime(u64)
}

#[derive(Deserialize)]
pub struct Manipulation {
    action: Type
}

async fn mine_one(client: &HttpClient) {
    client.request::<(), ArrayParams>(
        "katana_generateBlock",
        ArrayParams::default()
    ).await.expect("able to mine");
}

async fn mine_block(blocks: u64, client: &HttpClient) -> AxumJson<String> {
    for _ in 0..blocks  {
        mine_one(client).await
    }
    AxumJson(format!("Mined {} blocks", blocks))
}

async fn increase_block_time(seconds: u64, client: &HttpClient) -> AxumJson<String> {
    let mut params = ArrayParams::new();
    params.insert(seconds).expect("able to add seconds");
    client.request::<(), ArrayParams>(
        "katana_increaseNextBlockTimestamp",
        params
    ).await.expect("able to increase time");
    mine_one(client).await;
    AxumJson(format!("Increased block time by {} seconds", seconds))
}

pub async fn handler(
    Extension(state): Extension<ServerState>,
    Json(payload): Json<Manipulation>
) -> impl IntoResponse {
    let json_rpc_client = &state.json_rpc_client;
    match payload.action {
        Type::MineBlock(blocks) => mine_block(blocks, json_rpc_client).await,
        Type::IncreaseTime(seconds) => increase_block_time(seconds, json_rpc_client).await
    }
}