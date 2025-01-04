use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::net::SocketAddrV4;

use reqwest::Client;
use tokio::net::TcpListener;

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

struct App {
    address: SocketAddr,
}

impl App {
    async fn new() -> Self {
        let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0))
            .await
            .expect("Failed to start an app in test");
        let address = listener.local_addr().unwrap();
        tokio::spawn(zero2prod::run(listener));
        App { address }
    }

    fn url(&self, path: &str) -> String {
        format!("http://{}/{}", self.address, path.trim_start_matches("/"))
    }
}
