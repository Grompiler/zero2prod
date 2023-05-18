use crate::helpers::spawn_app;

#[tokio::test]
async fn should_be_success_when_health_check_works() {
    // Given
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let expected_body_size = Some(0);

    // When
    let response = client
        .get(format!("{}/health-check", app.address))
        .send()
        .await
        .expect("Failed to execute the request");

    // Then
    assert!(response.status().is_success());
    assert_eq!(expected_body_size, response.content_length());
}
