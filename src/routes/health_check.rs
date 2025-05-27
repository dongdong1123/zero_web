use actix_web::{HttpRequest, HttpResponse, Responder};
use std::net::TcpListener;

use crate::startup;

pub async fn health_check(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
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

pub fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind radom port");
    let port = listener.local_addr().unwrap().port();
    let server = startup::run(listener).expect("Failed to bind adress!");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
