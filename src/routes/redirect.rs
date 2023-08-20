use crate::{app_error::AppError, model::UrlRequestModel, AppState};
use axum::{
    extract::{Path, State},
    response::Redirect,
};
use sqlx::Error;
use std::sync::Arc;

pub async fn redirect(
    Path(extension): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<Redirect, AppError> {
    fetch_from_db(&extension, State(&data))
        .await
        .map(|model| Redirect::to(&model.url))
        .map_err(|e| AppError(e.into()))
}

async fn fetch_from_db(
    extension: &str,
    State(data): State<&Arc<AppState>>,
) -> Result<UrlRequestModel, Error> {
    sqlx::query_as!(
        UrlRequestModel,
        "SELECT * FROM briefly WHERE id = $1",
        extension
    )
    .fetch_one(&data.db)
    .await
}
