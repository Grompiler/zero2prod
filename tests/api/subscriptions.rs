use crate::helpers::spawn_app;

#[tokio::test]
async fn should_return_200_and_save_data_when_form_is_valid() {
    // Given
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let expected_status = 200;
    let body = "name=le%20guin&email=ursula%40gmail.com";

    let expected_email = "ursula@gmail.com";
    let expected_name = "le guin";

    // When
    let response = client
        .post(&format!("{}/subscribe", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute the request");

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscriptions");

    // Then
    assert_eq!(expected_status, response.status());
    assert_eq!(expected_email, saved.email);
    assert_eq!(expected_name, saved.name);
}

#[tokio::test]
async fn should_return_400_when_form_data_is_not_valid() {
    // Given
    let app = spawn_app().await;
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
            .post(&format!("{}/subscribe", &app.address))
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

#[tokio::test]
async fn should_return_400_when_fields_are_present_but_invalid() {
    // Given
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let expected_status = 400;
    let test_cases = vec![
        ("name=&email=ursula@gmail.com", "empty name"),
        ("name=le%40guin&email=", "empty email"),
        ("name=ursula&email=not_an_email", "invalid email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // When
        let response = client
            .post(&format!("{}/subscribe", &app.address))
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
