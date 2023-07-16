use axum::Json;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UrlRequest {
    url: String,
    extension: String,
}

pub async fn full_url(Json(payload): Json<UrlRequest>) {
    println!("{:?}, {:?}", payload.url, payload.extension);
}
