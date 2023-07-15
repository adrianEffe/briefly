use axum::Server;
use std::net::TcpListener;

pub struct TestApp {
    pub base_url: String,
    pub client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> TestApp {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Could not bind ephemeral socket");
        let port = listener.local_addr().unwrap().port();

        tokio::spawn(async move {
            let server = Server::from_tcp(listener)
                .unwrap()
                .serve(briefly::app().into_make_service());
            server.await.expect("server error");
        });

        TestApp {
            base_url: String::from(format!("http://127.0.0.1:{}/", port)),
            client: reqwest::Client::new(),
        }
    }

    pub fn get(&self, path: &str) -> reqwest::RequestBuilder {
        self.client.get(self.base_url.to_owned() + path)
    }

    pub fn post(&self, path: &str, body: String) -> reqwest::RequestBuilder {
        self.client
            .post(self.base_url.to_owned() + path)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
    }
}
