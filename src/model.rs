use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct UrlRequestModel {
    pub id: String,
    pub url: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct ShortenedUrlResponseModel {
    pub extension: String,
}
