pub mod shared;

use briefly::model::ShortenedUrlResponseModel;
use serde_json::from_slice;
use shared::test_app::TestApp;
use sqlx::{Connection, PgConnection};

#[tokio::test]
async fn shorten_returns_200_for_valid_form_data() {
    let app = TestApp::new().await;

    let mut connection = PgConnection::connect(&app.db_connection)
        .await
        .expect("Failed to connect to Postgres");

    let body = String::from("{\"url\":\"www.google.com\"}");
    let response = app.post("shorten", body).send().await.unwrap();

    assert!(response.status().is_success());

    let saved = sqlx::query!("SELECT url FROM briefly WHERE url = 'www.google.com'")
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved shortened url");

    assert_eq!(saved.url, "www.google.com");
}

#[tokio::test]
async fn shorten_with_extension_returns_200_for_valid_form_data() {
    let app = TestApp::new().await;

    let mut connection = PgConnection::connect(&app.db_connection)
        .await
        .expect("Failed to connect to Postgres");

    let url = "https://rust-lang.org";

    let custom_extension = "rustlang";

    let body = format!(
        "{{\"url\":\"{}\",\"extension\":\"{}\"}}",
        url, custom_extension
    );

    let _ = sqlx::query!("DELETE FROM briefly WHERE id = $1", custom_extension)
        .execute(&mut connection)
        .await;

    let response = app.post("shorten", body).send().await.unwrap();

    if response.status().is_success() {
        let response_body_bytes = response.bytes().await.unwrap().to_vec();
        let shortened_url_response: ShortenedUrlResponseModel =
            from_slice(&response_body_bytes).unwrap();
        assert_eq!(custom_extension, shortened_url_response.extension)
    } else {
        panic!("Test failed: Response status is not successful");
    }

    let saved = sqlx::query!("SELECT url FROM briefly WHERE id = $1", custom_extension)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved shortened url");

    assert_eq!(saved.url, url);
}

#[tokio::test]
async fn shorten_with_extension_returns_500_for_existing_record() {
    let app = TestApp::new().await;

    let url = "https://github.com";

    let custom_extension = "github";

    let body = format!(
        "{{\"url\":\"{}\",\"extension\":\"{}\"}}",
        url, custom_extension
    );

    let _ = app.post("shorten", body.clone()).send().await.unwrap();

    let response = app.post("shorten", body).send().await.unwrap();

    assert!(response.status().is_server_error());
}

#[tokio::test]
async fn shorten_returns_422_for_missing_data() {
    let app = TestApp::new().await;

    let test_case = ("{\"\":\"\"}", "missing the url");

    let response = app
        .post("shorten", test_case.0.to_string())
        .send()
        .await
        .unwrap();

    assert_eq!(
        422,
        response.status().as_u16(),
        "The API did not fail with 422 when the payload was {}.",
        test_case.1
    );
}

#[tokio::test]
async fn shorten_returns_400_for_bad_request() {
    let app = TestApp::new().await;

    let body = String::from("bad request");
    let response = app.post("shorten", body).send().await.unwrap();
    assert_eq!(
        400,
        response.status().as_u16(),
        "The API did not fail with 400."
    );
}
