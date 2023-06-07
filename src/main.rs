use axum::extract::Path;
use axum::routing::get;
use axum::Router;
use briefly::routes::{health_check};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/shorten/:user", get(shorten));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}

async fn shorten(Path(user): Path<String>) -> String {
    format!("Hello World from {}", user)
}
