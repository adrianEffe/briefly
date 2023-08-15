pub mod shared;

use briefly::{key_generator::generate, model::UrlRequestModel};
use chrono::Utc;
use reqwest::header::LOCATION;
use shared::test_app::TestApp;
use sqlx::{Connection, PgConnection};
use uuid::Uuid;

#[tokio::test]
async fn redirect_303_for_valid_redirection() {
    let app = TestApp::new().await;

    let mut connection = PgConnection::connect(&app.db_connection)
        .await
        .expect("Failed to connect to Postgres");

    let url = "https:://google.com";
    let uuid = Uuid::new_v4();
    let modifier = uuid.to_string() + url;
    let shortened = generate(&modifier);

    let url_model = sqlx::query_as!(
        UrlRequestModel,
        "INSERT INTO briefly (id, url, extension, created_at) VALUES ($1, $2, $3, $4) RETURNING *",
        uuid,
        url.to_string(),
        shortened,
        Utc::now()
    )
    .fetch_one(&mut connection)
    .await
    .unwrap();

    let response = app.get(&url_model.extension).send().await.unwrap();
    assert_eq!(303, response.status().as_u16());

    let location = response.headers().get(LOCATION).unwrap().to_str().unwrap();
    assert_eq!(url, location);
}
