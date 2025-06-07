use sqlx::PgPool;
use std::net::TcpListener;
use zero_web::configuration::get_configuration;
use zero_web::startup::run;
use zero_web::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero_web".into(), "info".into());
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let adress = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(adress).expect("Failed to bind radom port");
    run(listener, connection)?.await?;
    Ok(())
}
