use crate::{
    app::AppState, app_error::AppError, key_generator::generate, model::ShortenedUrlResponseModel,
    model::UrlRequestModel, schema::CreateShortUrlSchema,
};
use anyhow::anyhow;
use axum::{extract::State, Json};
use chrono::Utc;
use sqlx::Error;
use std::sync::Arc;

pub async fn shorten(
    State(data): State<Arc<AppState>>,
    Json(payload): Json<CreateShortUrlSchema>,
) -> Result<Json<ShortenedUrlResponseModel>, AppError> {
    let mut retry_count = 3;

    while retry_count > 0 {
        let query_result =
            insert_in_db(&payload.url, payload.extension.as_deref(), State(&data)).await;

        match query_result {
            Ok(request) => {
                let response = ShortenedUrlResponseModel {
                    extension: request.id,
                };
                return Ok(Json(response));
            }
            Err(e) => {
                if retry_count == 1 {
                    return Err(AppError(e.into()));
                }
                retry_count -= 1;
            }
        }
    }
    Err(AppError(anyhow!("Something went wrong")))
}

async fn insert_in_db(
    url: &str,
    extension: Option<&str>,
    State(data): State<&Arc<AppState>>,
) -> Result<UrlRequestModel, Error> {
    let date = Utc::now();
    let id = extension.map(|ext| ext.to_string()).or_else(|| {
        let modifier = format!("{}{}", date, url);
        Some(generate(&modifier))
    });

    sqlx::query_as!(
        UrlRequestModel,
        "INSERT INTO briefly (id, url, created_at) VALUES ($1, $2, $3) RETURNING *",
        id,
        url,
        date
    )
    .fetch_one(&data.db)
    .await
}
