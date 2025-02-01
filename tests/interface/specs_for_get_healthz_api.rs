use reqwest::StatusCode;

use crate::interface::system::system;
use crate::interface::system::System;

#[rstest::rstest]
#[tokio::test]
async fn health_check_returns_status_ok_and_no_content(#[future(awt)] system: System) {
    // Act
    let response = system.requestor.get_healthz().await;

    // Assert
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.content_length().unwrap(), 0);
}
