use actix_web::{HttpResponse, web};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

use crate::routes::spawn_app;

#[derive(serde::Deserialize)]
pub struct FromData {
    email: String,
    name: String,
}

pub async fn subcribe(from: web::Form<FromData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!("Adding a new subscriber.",
      %request_id,
      subscriber = %from.email,
      subscriber_name = %from.name
    );

    let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!("Saving new subscriber details in the database");

    // tracing::info!(
    //     "request_id {} -Adding '{}' '{}' as a new subscriber.",
    //     request_id,
    //     from.email,
    //     from.name
    // );
    // tracing::info!(
    //     "request_id {} -Saving new subscriber details in the database",
    //     request_id
    // );
    match sqlx::query!(
        r#"
      INSERT INTO subscriptions (id, email, name, subscribed_at)
      VALUES ($1, $2, $3, $4)
      "#,
        Uuid::new_v4(),
        from.email,
        from.name,
        Utc::now()
    )
    .execute(db_pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            // tracing::info!(
            //     "request_id {} -New subscriber details have been saved",
            //     request_id
            // );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
                "request_id {} -Failed to execute query: {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tokio::test]
async fn subcribe_return_a_400_when_data_is_missing() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();
    let test_class = vec![
        ("name=le%20", "missing the email"),
        ("email=usrla_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_class {
        let response = client
            .post(&format!("{}/subcriptions", &address.address))
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
