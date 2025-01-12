use reqwest::Client;

use crate::helper::TestApp;

#[tokio::test]
async fn health_check_returns_status_200_and_no_content() {
    // Arrange
    let app = TestApp::new().await;
    let client = Client::new();

    // Act
    let response = client
        .get(app.get_server_request_url("/healthz"))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
