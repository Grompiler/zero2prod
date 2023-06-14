use crate::helpers::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn should_be_logged_in_to_see_the_change_password_form() {
    // Given
    let app = spawn_app().await;
    let expected_redirect = "/login";

    // When
    let response = app.get_change_password().await;

    // Then
    assert_is_redirect_to(&response, expected_redirect);
}

#[tokio::test]
async fn should_be_logged_in_to_change_password() {
    // Given
    let app = spawn_app().await;
    let new_password = uuid::Uuid::new_v4().to_string();
    let expected_redirect = "/login";

    // When
    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": uuid::Uuid::new_v4().to_string(),
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;

    // Then
    assert_is_redirect_to(&response, expected_redirect);
}

#[tokio::test]
async fn should_match_when_entering_password_twice() {
    // Given
    let app = spawn_app().await;
    let new_password = uuid::Uuid::new_v4().to_string();
    let another_new_password = uuid::Uuid::new_v4().to_string();
    let expected_redirect = "/admin/password";

    // When
    app.post_login(&serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password
    }))
    .await;

    let reponse = app
        .post_change_password(&serde_json::json!({
            "current_password": &app.test_user.password,
            "new_password": new_password,
            "new_password_check": another_new_password,
        }))
        .await;

    // Then
    assert_is_redirect_to(&reponse, expected_redirect);

    let html_page = app.get_change_password_html().await;
    dbg!(&html_page);
    assert!(
        html_page.contains("You entered two different passwords - the field values must match.")
    );
}

#[tokio::test]
async fn should_redirect_when_password_is_invalid() {
    // Given
    let app = spawn_app().await;
    let new_password = uuid::Uuid::new_v4().to_string();
    let wrong_password = uuid::Uuid::new_v4().to_string();

    app.post_login(&serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password,
    }))
    .await;

    let _response = app
        .post_change_password(&serde_json::json!({
            "current_password": &wrong_password,
            "new_password": &new_password,
            "new_password_check": &new_password,

        }))
        .await;

    let html_page = app.get_change_password_html().await;
    assert!(html_page.contains("<p><i>The current password is incorrect.</i></p>"))
}
