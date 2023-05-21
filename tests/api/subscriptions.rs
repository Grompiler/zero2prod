use crate::helpers::spawn_app;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn subscribe_should_return_200_when_form_is_valid() {
    // Given
    let app = spawn_app().await;
    let expected_status = 200;
    let body = "name=le%20guin&email=ursula%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    // When
    let response = app.post_subscribe(body.into()).await;

    // Then
    assert_eq!(expected_status, response.status());
}

#[tokio::test]
async fn subscribe_should_persist_subscriber_when_form_is_valid() {
    // Given
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula%40gmail.com";

    let expected_email = "ursula@gmail.com";
    let expected_name = "le guin";
    let expected_status = "pending_confirmation";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    // When
    let _response = app.post_subscribe(body.into()).await;

    let saved = sqlx::query!("SELECT email, name, status FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscriptions");

    // Then
    assert_eq!(expected_email, saved.email);
    assert_eq!(expected_name, saved.name);
    assert_eq!(expected_status, saved.status);
}

#[tokio::test]
async fn subscribe_should_return_400_when_form_data_is_not_valid() {
    // Given
    let app = spawn_app().await;
    let expected_status = 400;
    let test_cases = vec![
        ("name=le%20guin", "missing email"),
        ("email=le%40guin", "missing name"),
        ("", "missing name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // When
        let response = app.post_subscribe(invalid_body.into()).await;

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
async fn subscribe_should_return_400_when_fields_are_present_but_invalid() {
    // Given
    let app = spawn_app().await;
    let expected_status = 400;
    let test_cases = vec![
        ("name=&email=ursula@gmail.com", "empty name"),
        ("name=le%40guin&email=", "empty email"),
        ("name=ursula&email=not_an_email", "invalid email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // When
        let response = app.post_subscribe(invalid_body.into()).await;

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
async fn subscribe_should_send_a_confirmation_email_for_valid_data() {
    // Given
    let app = spawn_app().await;
    let body = "name=le%guin&email=ursula%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    // When
    app.post_subscribe(body.into()).await;

    // Then
    // Mock asserts on drop
}

#[tokio::test]
async fn subscribe_should_send_a_confirmation_email_with_a_link_for_valid_subscription() {
    // Given
    let app = spawn_app().await;
    let body = "name=le%guin&email=ursula%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    // When
    app.post_subscribe(body.into()).await;
    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let body: serde_json::Value = serde_json::from_slice(&email_request.body).unwrap();
    let get_link = |s: &str| {
        let links: Vec<_> = linkify::LinkFinder::new()
            .links(s)
            .filter(|l| *l.kind() == linkify::LinkKind::Url)
            .collect();
        assert_eq!(links.len(), 1);
        links[0].as_str().to_owned()
    };

    // Then
    let html_link = get_link(body["HtmlBody"].as_str().unwrap());
    let text_link = get_link(body["TextBody"].as_str().unwrap());

    assert_eq!(html_link, text_link);
}
