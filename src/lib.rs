pub mod configuration;
pub mod routes;

use axum::routing::{get, post};
use axum::Router;
use configuration::get_configuration;
use routes::{full_url, health_check};

pub async fn run() {
    let app = app();
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}

pub fn app() -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/full_url", post(full_url))
}
