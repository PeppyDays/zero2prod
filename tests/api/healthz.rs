use reqwest::Client;

use crate::helper::App;

#[tokio::test]
async fn health_check_returns_status_200_and_no_content() {
    // Arrange
    let sut = App::new().await;
    let client = Client::new();

    // Act
    let response = client
        .get(sut.url("/healthz"))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
