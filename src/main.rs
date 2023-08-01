use briefly::{configuration::get_configuration, run};
use std::net::TcpListener;

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Could not bind to socket");
    run(listener, &connection_string).await
}
