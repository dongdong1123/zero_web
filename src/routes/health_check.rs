use actix_web::{HttpRequest, HttpResponse, Responder};
use sqlx::{Connection, PgConnection};
use std::net::TcpListener;

use crate::configuration::{self, get_configuration};
use crate::startup;

pub async fn health_check(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind radom port");
    let port = listener.local_addr().unwrap().port();
    let server = startup::run(listener).expect("Failed to bind adress!");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    let adress = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", &adress))
        .send()
        .await
        .expect("Failed to excute request.");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subcribe_return_a_200_for_valid_from_data() {
    let app_adress = spawn_app();
    let configuration = get_configuration().expect("Failed to read configuration!");
    let connect_string = configuration.database.connection_string();

    let mut connection = PgConnection::connect(&connect_string)
        .await
        .expect("Failed to connect to Postgres");

    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subcriptions", &app_adress))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request!");
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscriptions");
    // let name: String = saved.get("name");
    // let email: String = saved.get("email");
    // assert_eq!(email, "ursula_le_guin@gmail.com");
    // assert_eq!(name, "le guin");
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}
