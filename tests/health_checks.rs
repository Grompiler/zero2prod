use zero2prod::run;

#[tokio::test]
async fn test_health_check_works() {
    // Given
    spawn_app();
    let client = reqwest::Client::new();
    let response_body_size = Some(0);

    // When
    let response = client
        .get("http://localhost:8000/health-check")
        .send()
        .await
        .expect("Failed to execute the request");

    // Then
    assert!(response.status().is_success());
    assert_eq!(response_body_size, response.content_length());
}

fn spawn_app() {
    let server = run().expect("Failed to bind adress");
    let _ = tokio::spawn(server);
}
