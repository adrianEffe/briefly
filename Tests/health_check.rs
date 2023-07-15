use std::net::TcpListener;
use axum::Server;

pub struct TestApp {
    pub base_url: String,
    pub client: reqwest::Client
}

//TODO: assign random port for tests
impl TestApp {
    pub async fn new() -> TestApp {
        let listener = TcpListener::bind("0.0.0.0:3000").expect("Could not bind ephemeral socket");

        tokio::spawn(async move {
            let server = Server::from_tcp(listener).unwrap().serve(briefly::app()
                .into_make_service());
            server.await.expect("server error");
        });

        TestApp {
            base_url: String::from("http://0.0.0.0:3000/"),
            client: reqwest::Client::new(),
        }
    }

    pub fn get(&self, path: &str) -> reqwest::RequestBuilder {
        self.client.get(self.base_url.to_owned() + path)
    }
}

#[tokio::test]
async fn health_check_works() {
    let app = TestApp::new().await;
    let response = app.get("health_check").send().await.unwrap();

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
