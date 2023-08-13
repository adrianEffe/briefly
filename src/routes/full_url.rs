use crate::{
    key_generator::generate, model::UrlRequestModel, schema::CreateShortUrlSchema, AppState,
};
use axum::{extract::State, Json};
use chrono::Utc;
use sqlx::Error;
use std::sync::Arc;
use uuid::Uuid;

pub async fn full_url(
    State(data): State<Arc<AppState>>,
    Json(payload): Json<CreateShortUrlSchema>,
) {
    let mut retry_count = 3;

    while retry_count > 0 {
        let query_result = insert_in_db(&payload.url, State(data.clone())).await;

        match query_result {
            Ok(note) => {
                println!("okay received testing: {:?}", note);
                break;
            }
            Err(e) => {
                println!("failed with error: {:?}", e);
                //TODO : - For now assume is failing because of error code 23505, that stands for
                //duplicate key
                retry_count += 1;
            }
        }
    }
}

async fn insert_in_db(
    url: &str,
    State(data): State<Arc<AppState>>,
) -> Result<UrlRequestModel, Error> {
    let uuid = Uuid::new_v4();
    let modifier = uuid.to_string() + url;
    let shortened = generate(&modifier);

    sqlx::query_as!(
        UrlRequestModel,
        "INSERT INTO briefly (id, url, extension, created_at) VALUES ($1, $2, $3, $4) RETURNING *",
        uuid,
        url.to_string(),
        shortened,
        Utc::now()
    )
    .fetch_one(&data.db)
    .await
}
