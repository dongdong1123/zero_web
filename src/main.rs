use std::net::TcpListener;

use zero_web::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind radom port");
    run(listener)?.await
}
