use axum::Json;
use axum::response::IntoResponse;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct FundAddress {
    address: String,
    amount: f32,
    token_address: Option<String>
}

#[allow(dead_code)]
async fn increase_token() -> impl IntoResponse {
    todo!()
}

#[allow(dead_code)]
async fn increase_eth() -> impl IntoResponse {
    todo!()
}

pub async fn handler(Json(_payload): Json<FundAddress>) -> impl IntoResponse {
    // match payload.token_address {
    //     None => increase_eth(),
    //     Some(_) => increase_token()
    // }

    todo!()
}