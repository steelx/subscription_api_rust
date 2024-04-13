use std::net::TcpListener;

// Launch our application in the background ~somehow~
fn spawn_app() -> String {
  let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to get a random port!");
  let addr = listener.local_addr().unwrap().to_string();

  let server = subscription_api_rust::run(listener).expect("Faield to bind address");
  let _ = tokio::spawn(server);
  format!("http://{}", addr)
}

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn health_check_works() {
  let address = spawn_app();
  let client = reqwest::Client::new();

  // Act
  let response = client.get(format!("{}/health_check", address))
    .send()
    .await
    .expect("Failed to execute request!");

  assert!(response.status().is_success());
  assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_for_a_valid_form_data() {
  let address = spawn_app();
  let client = reqwest::Client::new();

  // Act
  let body = "name=Ajinkya%20Borade&email=ajinkya_123%40gmail.com";
  let response = client
    .post(format!("{}/subscriptions", address))
    .header("Content-Type", "application/x-www-form-urlencoded")
    .body(body)
    .send()
    .await
    .expect("Failed to post form data!");

  // Assert
  assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
  let address = spawn_app();
  let client = reqwest::Client::new();
  let test_cases = vec![
    ("name=ajinkya%20borade", "missing the email"),
    ("email=ajinkya_123%40gmail.com", "missing the name"),
    ("", "missing both name and email")
  ];

  for (body, error_message) in test_cases {
    // Act
    let response = client
      .post(format!("{}/subscriptions", address))
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
