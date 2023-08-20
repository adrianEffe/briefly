pub mod shared;

use briefly::{key_generator::generate, model::UrlRequestModel};
use chrono::Utc;
use reqwest::header::LOCATION;
use shared::test_app::TestApp;
use sqlx::{Connection, PgConnection};

#[tokio::test]
async fn redirect_303_for_valid_redirection() {
    let app = TestApp::new().await;

    let mut connection = PgConnection::connect(&app.db_connection)
        .await
        .expect("Failed to connect to Postgres");

    let url = "https:://google.com";
    let date = Utc::now();
    let modifier = date.to_string() + url;
    let shortened = generate(&modifier);

    let url_model = sqlx::query_as!(
        UrlRequestModel,
        "INSERT INTO briefly (id, url, created_at) VALUES ($1, $2, $3) RETURNING *",
        shortened,
        url.to_string(),
        Utc::now()
    )
    .fetch_one(&mut connection)
    .await
    .unwrap();

    let response = app.get(&url_model.id).send().await.unwrap();
    assert_eq!(303, response.status().as_u16());

    let location = response.headers().get(LOCATION).unwrap().to_str().unwrap();
    assert_eq!(url, location);
}

#[tokio::test]
async fn redirect_500_for_invalid_extension() {
    let app = TestApp::new().await;

    let response = app.get("-12-12-12").send().await.unwrap();
    assert_eq!(500, response.status().as_u16());
}
