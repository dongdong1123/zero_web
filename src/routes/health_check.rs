use actix_web::{HttpRequest, HttpResponse, Responder};
use once_cell::sync::Lazy;
use secrecy::ExposeSecret;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

use crate::configuration::{DatabaseSettings, get_configuration};
use crate::startup;
use crate::telemetry::{get_subscriber, init_subscriber};

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".into();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn health_check(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind radom port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    let mut configuration = get_configuration().expect("Failed to read configuration");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let connect_pool = configuration_database(&configuration.database).await;
    let server = startup::run(listener, connect_pool.clone()).expect("Failed to bind adress!");
    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: connect_pool,
    }
}

pub async fn configuration_database(config: &DatabaseSettings) -> PgPool {
    let mut connection =
        PgConnection::connect(&config.connection_string_without_db().expose_secret())
            .await
            .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");
    let connection_pool = PgPool::connect(&config.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate database");
    connection_pool
}

#[tokio::test]
async fn health_check_works() {
    let adress = spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", &adress.address))
        .send()
        .await
        .expect("Failed to excute request.");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subcribe_return_a_200_for_valid_from_data() {
    let app_adress = spawn_app().await;
    // let configuration = get_configuration().expect("Failed to read configuration!");
    // let connect_string = configuration.database.connection_string();

    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subcriptions", &app_adress.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request!");
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app_adress.db_pool)
        .await
        .expect("Failed to fetch saved subscriptions");
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}
