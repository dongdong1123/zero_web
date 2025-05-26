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

fn spawn_app() -> String {
    use std::net::TcpListener;
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind radom port");
    let port = listener.local_addr().unwrap().port();
    let server = zero_web::run(listener).expect("Failed to bind adress!");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
