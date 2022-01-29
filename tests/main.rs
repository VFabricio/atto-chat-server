use atto_chat_server::application::Application;
use reqwest::Client;

#[tokio::test]
async fn health_check_works() {
    let application = Application::new("127.0.0.1", 0).unwrap();
    let port = application.port();
    tokio::spawn(async {
        application.start(false).await.unwrap();
    });

    let client = Client::new();
    let response = client
        .get(&format!("http://127.0.0.1:{}/health_check", port))
        .send()
        .await
        .expect("Failed to execute query");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn non_existing_route_returns_404() {
    let application = Application::new("127.0.0.1", 0).unwrap();
    let port = application.port();
    tokio::spawn(async {
        application.start(false).await.unwrap();
    });

    let client = Client::new();
    let response = client
        .get(&format!("http://127.0.0.1:{}/i_do_not_exist", port))
        .send()
        .await
        .expect("Failed to execute query");

    assert_eq!(response.status(), 404);
    assert_eq!(Some(0), response.content_length());
}
