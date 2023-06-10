use crate::helpers::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn should_set_an_error_flash_message_on_failure() {
    // Given
    let app = spawn_app().await;
    let login_body = serde_json::json!({
        "username": "random-username",
        "password": "random-password"
    });
    let expected_status = 303;

    // When
    let response = app.post_login(&login_body).await;

    // Then
    assert_eq!(expected_status, response.status());
    assert_is_redirect_to(&response, "/login");

    let html_page = app.get_login_html().await;
    assert!(html_page.contains(r#"<p><i>Authentication failed</i></p>"#));

    let html_page = app.get_login_html().await;
    assert!(!html_page.contains("Authentication failed"));
}
