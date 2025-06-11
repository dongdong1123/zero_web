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

#[tracing::instrument(
    name = "Adding a new subscriber.",
    skip(from, db_pool),
    fields(
      subscriber = %from.email,
      subscriber_name = %from.name
    )
)]
pub async fn subcribe(from: web::Form<FromData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    let query_span = tracing::info_span!("Saving new subscriber details in the database");
    match insert_subscriber(&pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().finish(),
    }
}

#[tarcing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, db_pool)
)]
pub async fn insert_subscriber(pool: &PgPool, form: &FromData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
      INSERT INTO subscriptions (id, email, name, subscribed_at)
      VALUES ($1, $2, $3, $4)
      "#,
        Uuid::new_v4(),
        form.email(),
        form.name(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map(|_| ())
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
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
