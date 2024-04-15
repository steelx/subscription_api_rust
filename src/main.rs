use std::net::TcpListener;
use sqlx::{PgPool};
use subscription_api_rust::configuration::get_configuration;
use subscription_api_rust::startup;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration!");
    let connection_string = configuration.database.connection_string();
    let connection = PgPool::connect(&connection_string)
      .await.expect("Failed to connect to Postgres");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    startup::run(listener, connection)?.await
}
