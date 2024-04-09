
// Launch our application in the background ~somehow~
fn spawn_app() {
  let server = subscription_api_rust::run().expect("Faield to bind address");

  let _ = tokio::spawn(server);
}

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn health_check_works() {
  spawn_app();

  let client = reqwest::Client::new();

  // Act
  let response = client.get("http://127.0.0.0:8000/health_check")
    .send()
    .await
    .expect("Failewd to execute request!");

  assert!(response.status().is_success());
  assert_eq!(Some(0), response.content_length());
}