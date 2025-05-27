use actix_web::{HttpResponse, web};

use crate::routes::spawn_app;

#[derive(serde::Deserialize)]
pub struct FromData {
    email: String,
    name: String,
}

pub async fn subcribe(_from: web::Form<FromData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[tokio::test]
async fn subcribe_return_a_200_for_valid_from_data() {
    let app_adress = spawn_app();
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
}

#[tokio::test]
async fn subcribe_return_a_400_when_data_is_missing() {
    let app_adress = spawn_app();
    let client = reqwest::Client::new();
    let test_class = vec![
        ("name=le%20", "missing the email"),
        ("email=usrla_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_class {
        let response = client
            .post(&format!("{}/subcriptions", &app_adress))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request!");
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}",
            error_message
        );
    }
}
