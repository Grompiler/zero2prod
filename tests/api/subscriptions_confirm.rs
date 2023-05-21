use crate::helpers::spawn_app;

#[tokio::test]
async fn should_reject_with_400_when_confirmation_is_without_token() {
    // Given
    let app = spawn_app().await;
    let expected_status = 400;

    // When
    let response = reqwest::get(&format!("{}/subscriptions/confirm", app.address))
        .await
        .unwrap();

    // Then
    assert_eq!(expected_status, response.status())
}
