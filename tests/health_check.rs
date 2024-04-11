use std::net::TcpListener;

// Launch our application in the background ~somehow~
fn spawn_app() -> String {
  let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to get a random port!");
  let addr = listener.local_addr().unwrap().to_string();

  let server = subscription_api_rust::run(listener).expect("Faield to bind address");
  let _ = tokio::spawn(server);
  addr
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
  let response = client.get(format!("http://{}/health_check", address))
    .send()
    .await
    .expect("Failed to execute request!");

  assert!(response.status().is_success());
  assert_eq!(Some(0), response.content_length());
}