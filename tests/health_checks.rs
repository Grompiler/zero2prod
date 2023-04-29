use std::net::TcpListener;
use zero2prod::run;

#[tokio::test]
async fn should_be_success_when_health_check_works() {
    // Given
    let address = spawn_app();
    let client = reqwest::Client::new();
    let expected_body_size = Some(0);

    // When
    let response = client
        .get(format!("{}/health-check", address))
        .send()
        .await
        .expect("Failed to execute the request");

    // Then
    assert!(response.status().is_success());
    assert_eq!(expected_body_size, response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind adress");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{port}")
}

#[tokio::test]
async fn should_return_200_when_form_data_is_valid() {
    // Given
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let expected_status = 200;
    let body = "name=le%20guin&email=ursula%40@gmail.com";

    // When
    let response = client
        .post(&format!("{}/subsribe", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute the request");

    // Then
    assert_eq!(expected_status, response.status());
}

#[tokio::test]
async fn should_return_400_when_form_data_is_not_valid() {
    // Given
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let expected_status = 400;
    let test_cases = vec![
        ("name=le%20guin", "missing email"),
        ("email=le%40guin", "missing name"),
        ("", "missing name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // When
        let response = client
            .post(&format!("{}/subsribe", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute the request");

        // Then
        assert_eq!(
            expected_status,
            response.status(),
            "The api did not fail with 400 Bad Request when the payload was {}",
            error_message
        );
    }
}
