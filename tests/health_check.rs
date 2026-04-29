use std::net::TcpListener;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("failed to bind random port");

    let port = listener.local_addr().unwrap().port();
    let server = rust_prod::run(listener).expect("failed to bind address");

    let _ = actix_web::rt::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[actix_web::test]
async fn health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute reqwest.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
