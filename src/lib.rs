pub mod configuration;
pub mod routes;

use axum::routing::{get, post};
use axum::Router;
use configuration::get_configuration;
use routes::{full_url, health_check};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::sync::Arc;

pub async fn run() {
    let configuration = get_configuration().expect("Failed to read configuration");

    let pool = connect_to_database(configuration.database.connection_string())
        .await
        .expect("Failed to conect to the database");
    let app_state = Arc::new(AppState { db: pool.clone() });
    let address = format!("127.0.0.1:{}", configuration.application_port);

    let app = app(app_state);

    axum::Server::bind(&address.parse().unwrap())
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
        .with_state(app_state)
}

async fn connect_to_database(connection_string: String) -> Option<Pool<Postgres>> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(&connection_string)
        .await
        .ok()
}
