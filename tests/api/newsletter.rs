use crate::helpers::{assert_is_redirect_to, spawn_app, ConfirmationsLinks, TestApp};
use wiremock::matchers::{any, method, path};
use wiremock::{Mock, ResponseTemplate};

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

#[tokio::test]
async fn should_not_send_newsletter_to_unconfirmed_subscribers() {
    // Given
    let app = spawn_app().await;
    create_unconfirmed_subscriber(&app).await;
    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "text_content": "Newsletter body as plain text",
        "html_content": "<p>Newsletter body as html</p>",
        "idempotency_key": uuid::Uuid::new_v4().to_string(),
    });
    let expected_redirect = "/admin/newsletters";

    Mock::given(any())
        .respond_with(ResponseTemplate::new(200))
        .expect(0)
        .mount(&app.email_server)
        .await;

    app.test_user.login(&app).await;

    // When
    let response = app.post_publish_newsletters(&newsletter_request_body).await;
    assert_is_redirect_to(&response, expected_redirect);
    // Then
    let html_page = app.get_publish_newsletter_html().await;
    assert!(html_page.contains("<p><i>The newsletter issue has been published!</i></p>"));
}

#[tokio::test]
async fn should_send_newsletter_to_confirmed_subscribers() {
    // Given
    let app = spawn_app().await;
    create_confirmed_subsriber(&app).await;
    let expected_redirect = "/admin/newsletters";
    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "text_content": "Newsletter body as plain text",
        "html_content": "<p>Newsletter body as html</p>",
        "idempotency_key": uuid::Uuid::new_v4().to_string(),
    });

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    app.test_user.login(&app).await;
    // When
    let response = app.post_publish_newsletters(&newsletter_request_body).await;
    assert_is_redirect_to(&response, expected_redirect);

    // Then
    let html_page = app.get_publish_newsletter_html().await;
    assert!(html_page.contains("<p><i>The newsletter issue has been published!</i></p>"));
}

#[tokio::test]
async fn should_be_logged_in_to_see_the_newsletter_form() {
    let expected_redirect = "/login";
    // Given
    let app = spawn_app().await;

    // When
    let response = app.get_publish_newsletters().await;

    // Then
    assert_is_redirect_to(&response, expected_redirect);
}

#[tokio::test]
async fn should_be_logged_in_to_publish_a_newsletter() {
    // Given
    let app = spawn_app().await;
    let expected_redirect = "/login";

    // When
    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "text_content": "Newsletter body as plain text",
        "html_content": "<p>Newsletter body as HTML</p>",
        "idempotency_key": uuid::Uuid::new_v4().to_string(),
    });
    let response = app.post_publish_newsletters(&newsletter_request_body).await;

    // Then
    assert_is_redirect_to(&response, expected_redirect);
}

#[tokio::test]
async fn should_be_idempotent_when_creating_newsletter() {
    // Given
    let app = spawn_app().await;
    let expected_redirect = "/admin/newsletters";
    create_confirmed_subsriber(&app).await;
    app.test_user.login(&app).await;

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    // When
    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "text_content": "Newsletter body as plain text",
        "html_content": "<p>Newsletter body as HTML</p>",
        "idempotency_key": uuid::Uuid::new_v4().to_string(),

    });

    // Then
    let response = app.post_publish_newsletters(&newsletter_request_body).await;
    assert_is_redirect_to(&response, expected_redirect);
    let html_page = app.get_publish_newsletter_html().await;
    assert!(html_page.contains("<p><i>The newsletter issue has been published!</i></p>"));

    let response = app.post_publish_newsletters(&newsletter_request_body).await;
    assert_is_redirect_to(&response, expected_redirect);
    let html_page = app.get_publish_newsletter_html().await;
    assert!(html_page.contains("<p><i>The newsletter issue has been published!</i></p>"));
}
