use axum::extract::Path;
use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/shorten:user", get(shorten));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}

async fn shorten(Path(user): Path<String>) -> String {
    format!("Hello World from {}", user)
}
