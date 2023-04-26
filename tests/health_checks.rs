use std::net::TcpListener;
use zero2prod::run;

#[tokio::test]
async fn test_health_check_works() {
    // Given
    let address = spawn_app();
    let client = reqwest::Client::new();
    let response_body_size = Some(0);

    // When
    let response = client
        .get(format!("{}/health-check", address))
        .send()
        .await
        .expect("Failed to execute the request");

    // Then
    assert!(response.status().is_success());
    assert_eq!(response_body_size, response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind adress");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{port}")
}
