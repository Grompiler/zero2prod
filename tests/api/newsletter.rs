use crate::helpers::{spawn_app, ConfirmationsLinks, TestApp};
use wiremock::matchers::{any, method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn should_not_send_newsletter_to_unconfirmed_subscribers() {
    // Given
    let app = spawn_app().await;
    create_unconfirmed_subscriber(&app).await;
    let expected_status = 200;
    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "content":{
            "text": "Newsletter body as plain text",
            "html": "<p>Newsletter body as html</p>",
        }
    });

    Mock::given(any())
        .respond_with(ResponseTemplate::new(200))
        .expect(0)
        .mount(&app.email_server)
        .await;

    // When
    let response = app.post_newsletters(newsletter_request_body).await;

    // Then
    assert_eq!(expected_status, response.status());
}

#[tokio::test]
async fn should_send_newsletter_to_confirmed_subscribers() {
    // Given
    let app = spawn_app().await;
    create_confirmed_subsriber(&app).await;
    let expected_status = 200;
    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "content":{
            "text": "Newsletter body as plain text",
            "html": "<p>Newsletter body as html</p>",
        }
    });

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    // When
    let response = app.post_newsletters(newsletter_request_body).await;

    // Then
    assert_eq!(expected_status, response.status());
}

#[tokio::test]
async fn should_return_400_for_invalid_data() {
    // Given
    let app = spawn_app().await;
    let expected_status = 400;
    let test_cases = vec![
        (
            serde_json::json!({
                "content":{
                    "text": "Newsletter body as plain text",
                    "html": "<p>Newsletter body as html</p>",
                }
            }),
            "missing title",
        ),
        (
            serde_json::json!({"title": "Nesletter!"}),
            "missing content",
        ),
    ];
    // When
    // Then
    for (invalid_body, error_message) in test_cases {
        let response = app.post_newsletters(invalid_body).await;
        assert_eq!(
            expected_status,
            response.status(),
            "The API did not fail with 400 when the payload was {}.",
            error_message
        );
    }
}

#[tokio::test]
async fn should_reject_when_authorization_is_missing() {
    // Given
    let app = spawn_app().await;
    let expected_status = 401;
    let expected_auth_header = r#"Basic realm="publish""#;

    // When
    let response = reqwest::Client::new()
        .post(&format!("{}/newsletters", &app.address))
        .json(&serde_json::json!({
                    "title": "Newsletter title",
                    "content": {
                        "text": "Newsletter body as plain text",
                        "html": "<p>Newsletter body as html</p>"
                    }
        }))
        .send()
        .await
        .expect("Failed to execute the request");

    // Then
    assert_eq!(expected_status, response.status());
    assert_eq!(expected_auth_header, response.headers()["WWW-Authenticate"]);
}

async fn create_unconfirmed_subscriber(app: &TestApp) -> ConfirmationsLinks {
    let body = "name=le%20guin&email=ursula%40gmail.com";

    let _mock_guard = Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .named("Create unconfirmed subscriber")
        .expect(1)
        .mount_as_scoped(&app.email_server)
        .await;

    app.post_subscriptions(body.into())
        .await
        .error_for_status()
        .unwrap();

    let email_request = &app
        .email_server
        .received_requests()
        .await
        .unwrap()
        .pop()
        .unwrap();
    app.get_confirmation_links(email_request)
}

async fn create_confirmed_subsriber(app: &TestApp) {
    let confirmation_link = create_unconfirmed_subscriber(app).await;
    reqwest::get(confirmation_link.html)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();
}
