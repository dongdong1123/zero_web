use std::net::TcpListener;
use zero_web::configuration::get_configuration;
use zero_web::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let adress = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(adress).expect("Failed to bind radom port");
    run(listener)?.await
}
