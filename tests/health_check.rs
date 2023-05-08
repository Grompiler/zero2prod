use once_cell::sync::Lazy;
use secrecy::ExposeSecret;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::configuration::{get_configuration, DatabaseSettings};
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

struct TestApp {
    address: String,
    db_pool: PgPool,
}
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "zero2prod".into();
    let subscriber_name = "info".into();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let mut database = get_configuration()
        .expect("Failed to read configuration.")
        .database;
    database.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(&database).await;
    let server = run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address: format!("http://127.0.0.1:{port}"),
        db_pool: connection_pool,
    }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection =
        PgConnection::connect(&config.connection_string_without_db_name().expose_secret())
            .await
            .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create the database");

    let connection_pool = PgPool::connect(&config.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

#[tokio::test]
async fn should_be_success_when_health_check_works() {
    // Given
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let expected_body_size = Some(0);

    // When
    let response = client
        .get(format!("{}/health-check", app.address))
        .send()
        .await
        .expect("Failed to execute the request");

    // Then
    assert!(response.status().is_success());
    assert_eq!(expected_body_size, response.content_length());
}

#[tokio::test]
async fn should_return_200_and_save_data_when_form_is_valid() {
    // Given
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let expected_status = 200;
    let body = "name=le%20guin&email=ursula%40gmail.com";

    let expected_email = "ursula@gmail.com";
    let expected_name = "le guin";

    // When
    let response = client
        .post(&format!("{}/subscribe", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute the request");

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscriptions");

    // Then
    assert_eq!(expected_status, response.status());
    assert_eq!(expected_email, saved.email);
    assert_eq!(expected_name, saved.name);
}

#[tokio::test]
async fn should_return_400_when_form_data_is_not_valid() {
    // Given
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let expected_status = 400;
    let test_cases = vec![
        ("name=le%20guin", "missing email"),
        ("email=le%40guin", "missing name"),
        ("", "missing name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // When
        let response = client
            .post(&format!("{}/subscribe", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute the request");

        // Then
        assert_eq!(
            expected_status,
            response.status(),
            "The api did not fail with 400 Bad Request when the payload was {}",
            error_message
        );
    }
}
