use briefly::{app::run, configuration::get_configuration};
use std::net::TcpListener;
use std::sync::Arc;

pub struct TestApp {
    pub base_url: String,
    pub client: reqwest::Client,
    pub db_connection: Arc<String>,
}

impl TestApp {
    pub async fn new() -> TestApp {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Could not bind ephemeral socket");
        let port = listener.local_addr().unwrap().port();

        let configuration = get_configuration().expect("Failed to read configuration");
        let connection_string = Arc::new(configuration.database.connection_string());
        let db_connection = Arc::clone(&connection_string);

        tokio::spawn(async move {
            run(listener, &connection_string).await;
        });

        TestApp {
            base_url: format!("http://127.0.0.1:{}/", port),
            client: reqwest::Client::new(),
            db_connection,
        }
    }

    pub fn get(&self, path: &str) -> reqwest::RequestBuilder {
        self.client.get(self.base_url.to_owned() + path)
    }

    pub fn post(&self, path: &str, body: String) -> reqwest::RequestBuilder {
        self.client
            .post(self.base_url.to_owned() + path)
            .header("Content-Type", "application/json")
            .body(body)
    }
}
