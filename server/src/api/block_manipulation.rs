use axum::Json;
use axum::response::IntoResponse;
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::params::ArrayParams;
use jsonrpsee_http_client::{HttpClient, HttpClientBuilder};
use serde::Deserialize;

#[derive(Deserialize)]
enum Type {
    MineBlock(u64),
    IncreaseTime(u64)
}

#[derive(Deserialize)]
pub struct Manipulation {
    action: Type
}

async fn mine_one(client: HttpClient) {
    client.request::<(), ArrayParams>(
        "katana_generateBlock",
        ArrayParams::default()
    ).await.expect("able to mine");
}

async fn mine_block(blocks: u64, client: HttpClient) {
    for _ in 0..blocks  {
        mine_one(client.clone()).await
    }
}

async fn increase_block_time(seconds: u64, client: HttpClient) {
    let mut params = ArrayParams::new();
    params.insert(seconds).expect("able to add seconds");
    client.request::<(), ArrayParams>(
        "katana_increaseNextBlockTimestamp",
        params
    ).await.expect("able to increase time");
    mine_one(client.clone()).await
}

pub async fn handler(
    Json(payload): Json<Manipulation>
) -> impl IntoResponse {

    // TODO figure out how to use the extension layer to share the jsonRpcClient
    let json_rpc_client = HttpClientBuilder::default()
        .build("http://0.0.0.0:5050")
        .unwrap();

    match payload.action {
        Type::MineBlock(blocks) => mine_block(blocks, json_rpc_client).await,
        Type::IncreaseTime(seconds) => increase_block_time(seconds, json_rpc_client).await
    }
}