pub mod configuration;
pub mod model;
pub mod routes;
pub mod schema;

use axum::routing::{get, post};
use axum::Router;
use routes::{full_url, health_check};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::net::TcpListener;
use std::sync::Arc;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

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

pub struct AppState {
    db: Pool<Postgres>,
}

pub fn app(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/full_url", post(full_url))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(app_state)
}

async fn connect_to_database(connection_string: &str) -> Option<Pool<Postgres>> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(connection_string)
        .await
        .ok()
}
