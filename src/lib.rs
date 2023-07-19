pub mod configuration;
pub mod routes;

use axum::routing::{get, post};
use axum::Router;
use routes::{full_url, health_check};

pub async fn run() {
    let app = app();

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}

pub fn app() -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/full_url", post(full_url))
}
