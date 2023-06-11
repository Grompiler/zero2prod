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
