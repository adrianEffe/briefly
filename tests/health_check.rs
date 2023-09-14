pub mod shared;

use shared::test_app::TestApp;

#[tokio::test]
async fn health_check_works() {
    let app = TestApp::new().await;
    let response = app.get("health_check").send().await.unwrap();

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn health_check_works_for_random_port() {
    let app = TestApp::new().await;
    let response = app.get("health_check").send().await.unwrap();

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
