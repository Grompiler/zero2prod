use crate::helpers::spawn_app;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

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

#[tokio::test]
async fn should_return_200_when_calling_subscribe_link() {
    // Given
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula%40gmail.com";
    let expected_status = 200;

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    // When
    app.post_subscribe(body.into()).await;
    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_confirmation_links(email_request);
    let response = reqwest::get(confirmation_links.html).await.unwrap();

    // Then
    assert_eq!(expected_status, response.status())
}

#[tokio::test]
async fn clicking_on_the_confirmation_link_confirms_a_subscriber() {
    // Given
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula%40gmail.com";
    let expected_name = "le guin";
    let expected_email = "ursula@gmail.com";
    let expected_status = "confirmed";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    // When
    app.post_subscribe(body.into()).await;
    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_confirmation_links(email_request);
    let _response = reqwest::get(confirmation_links.html)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();

    // Then
    let saved = sqlx::query!("SELECT email, name, status FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(expected_email, saved.email);
    assert_eq!(expected_name, saved.name);
    assert_eq!(expected_status, saved.status);
}
