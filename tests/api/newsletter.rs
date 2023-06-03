use crate::helpers::{spawn_app, TestApp};
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
    let response = reqwest::Client::new()
        .post(&format!("{}/newsletters", &app.address))
        .json(&newsletter_request_body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Then
    assert_eq!(expected_status, response.status());
}

async fn create_unconfirmed_subscriber(app: &TestApp) {
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
}
