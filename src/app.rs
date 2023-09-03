use crate::routes::{health_check, redirect, shorten};
use axum::routing::{get, post};
use axum::Router;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::net::TcpListener;
use std::sync::Arc;

pub struct AppState {
    pub db: PgPool,
}

pub async fn run(listener: TcpListener, pool: PgPool) {
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
        .with_state(app_state)
}

pub async fn connect_to_database(connection_string: &str) -> Option<PgPool> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(connection_string)
        .await
        .ok()
}
