pub mod shared;

use briefly::configuration::get_configuration;
use shared::test_app::TestApp;
use sqlx::{Connection, PgConnection};

#[tokio::test]
async fn full_url_returns_200_for_valid_form_data() {
    let app = TestApp::new().await;
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();

    let connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres");

    let body = String::from("{\"url\":\"www.google.com\",\"extension\":\"google\"}");
    let response = app.post("full_url", body).send().await.unwrap();

    assert!(response.status().is_success());
}

#[tokio::test]
async fn full_url_returns_422_for_missing_data() {
    let app = TestApp::new().await;

    let test_cases = vec![
        ("{\"url\":\"www.google.com\"}", "missing the extension"),
        ("{\"extension\":\"google\"}", "missing the url"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app
            .post("full_url", invalid_body.to_string())
            .send()
            .await
            .unwrap();

        assert_eq!(
            422,
            response.status().as_u16(),
            "The API did not fail with 422 when the payload was {}.",
            error_message
        );
    }
}

#[tokio::test]
async fn full_url_returns_400_for_bad_request() {
    let app = TestApp::new().await;

    let body = String::from("bad request");
    let response = app.post("full_url", body).send().await.unwrap();
    assert_eq!(
        400,
        response.status().as_u16(),
        "The API did not fail with 400."
    );
}
