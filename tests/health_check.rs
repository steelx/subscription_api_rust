use std::net::TcpListener;
use sqlx::{Executor, PgPool};
use uuid::Uuid;
use subscription_api_rust::configuration::{DatabaseSettings, get_configuration};

pub struct TestApp {
  pub address: String,
  pub db_pool: PgPool
}

// Launch our application in the background ~somehow~
async fn spawn_app() -> TestApp {
  let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to get a random port!");
  let addr = listener.local_addr().unwrap().to_string();

  let mut configuration = get_configuration().expect("Failed to read configuration.");
  configuration.database.database_name = Uuid::new_v4().to_string();
  // randomize database so we migrate original one to `./migrations`
  let connection = configure_database(&configuration.database).await;

  let server = subscription_api_rust::startup::run(listener, connection.clone()).expect("Failed to bind address");
  let _ = tokio::spawn(server);

  TestApp {
    address: format!("http://{}", addr),
    db_pool: connection
  }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
  // Create database
  let connection_string = &config.connection_string_without_database_name();
  let connection = PgPool::connect(&connection_string)
    .await.expect("Failed to connect to Postgres");

  connection
    .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
    .await
    .expect("Failed to create test database");

  // Migrate database
  let connection_pool = PgPool::connect(&config.connection_string())
    .await.expect("Failed to connect to existing database!");

  sqlx::migrate!("./migrations")
    .run(&connection_pool)
    .await.expect("Failed to migrate database");

  connection_pool
}

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn health_check_works() {
  let app = spawn_app().await;
  let client = reqwest::Client::new();

  // Act
  let response = client.get(format!("{}/health_check", &app.address))
    .send()
    .await
    .expect("Failed to execute request!");

  assert!(response.status().is_success());
  assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_for_a_valid_form_data() {
  let app = spawn_app().await;
  let client = reqwest::Client::new();

  dbg!(&app.address);

  // Act
  let body = "name=Ajinkya%20Borade&email=ajinkya_123%40gmail.com";
  let response = client
    .post( format!("{}/subscriptions", &app.address))
    .header("Content-Type", "application/x-www-form-urlencoded")
    .body(body)
    .send()
    .await
    .expect("Failed to post form data!");

  // Assert
  assert_eq!(200, response.status().as_u16());

  // let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
  //   .fetch_one(&app.db_pool)
  //   .await
  //   .expect("Failed to fetch saved Subscriptions from database");
  //
  // dbg!(&saved);
  //
  // assert_eq!(saved.email, "ajinkya_123@gmail.com");
  // assert_eq!(saved.name, "Ajinkya Borade");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
  let app = spawn_app().await;
  let client = reqwest::Client::new();
  let test_cases = vec![
    ("name=ajinkya%20borade", "missing the email"),
    ("email=ajinkya_123%40gmail.com", "missing the name"),
    ("", "missing both name and email")
  ];

  for (body, error_message) in test_cases {
    // Act
    let response = client
      .post(format!("{}/subscriptions", &app.address))
      .header("Content-Type", "application/x-www-form-urlencoded")
      .body(body)
      .send()
      .await
      .expect("Failed to post form data!");

    // Assert
    assert_eq!(
      400,
      response.status().as_u16(),
      // API should fail
      "The API did not fail with 400 Bad Request when the payload was {}",
      error_message
    );
  }
}
