use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct UrlRequestModel {
    pub id: Uuid,
    pub url: String,
    pub extension: String,
    pub created_at: Option<DateTime<Utc>>,
}
