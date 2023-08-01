use crate::{model::UrlRequestModel, schema::CreateShortUrlSchema, AppState};
use axum::{extract::State, Json};
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

pub async fn full_url(
    State(data): State<Arc<AppState>>,
    Json(payload): Json<CreateShortUrlSchema>,
) {
    let query_result = sqlx::query_as!(
        UrlRequestModel,
        "INSERT INTO briefly (id, url, extension, created_at) VALUES ($1, $2, $3, $4) RETURNING *",
        Uuid::new_v4(),
        payload.url.to_string(),
        Uuid::new_v4().to_string(), // TODO: - handle extension to be shortened url extension
        Utc::now()
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(note) => {
            println!("okay received testing: {:?}", note);
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
