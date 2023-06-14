use crate::helpers::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn should_redirect_when_accessing_admin_dashboard_if_not_logged_in() {
    // Given
    let app = spawn_app().await;

    // When
    let response = app.get_admin_dashboard().await;

    // Then
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn should_clear_session_state_on_logout() {
    // Given
    let app = spawn_app().await;
    let login_body = serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password,
    });
    let admin_dashboard_uri = "/admin/dashboard";
    let login_uri = "/login";
    let response = app.post_login(&login_body).await;
    assert_is_redirect_to(&response, admin_dashboard_uri);

    let html_page = app.get_admin_dashboard_html().await;
    assert!(html_page.contains(&format!("Welcome {}", app.test_user.username)));

    let response = app.post_logout().await;
    assert_is_redirect_to(&response, login_uri);

    let html_page = app.get_login_html().await;
    assert!(html_page.contains(r#"<p><i>You have successfully logged out.</i></p>"#));

    let response = app.get_admin_dashboard().await;
    assert_is_redirect_to(&response, login_uri);
}
