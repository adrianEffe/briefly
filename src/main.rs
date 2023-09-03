use briefly::app::{app, AppState};
use shuttle_shared_db::Postgres;
use sqlx::PgPool;
use std::sync::Arc;
use tracing::info;

#[shuttle_runtime::main]
pub async fn axum(#[Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    info!("Running database migration");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Migrations failed :(");

    let app_state = Arc::new(AppState { db: pool.clone() });
    let app = app(app_state);

    Ok(app.into())
}
