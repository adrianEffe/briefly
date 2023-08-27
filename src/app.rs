use crate::{
    request_tracing::RequestSpan,
    routes::{health_check, redirect, shorten},
};
use axum::routing::{get, post};
use axum::Router;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::net::TcpListener;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

pub struct AppState {
    pub db: Pool<Postgres>,
}

pub async fn run(listener: TcpListener, db_connection: &str) {
    let pool = connect_to_database(db_connection)
        .await
        .expect("Failed to conect to the database");
    let app_state = Arc::new(AppState { db: pool.clone() });

    let app = app(app_state);
    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap()
}

pub fn app(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/shorten", post(shorten))
        .route("/:extension", get(redirect))
        .layer(TraceLayer::new_for_http().make_span_with(RequestSpan))
        .with_state(app_state)
}

async fn connect_to_database(connection_string: &str) -> Option<Pool<Postgres>> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(connection_string)
        .await
        .ok()
}
