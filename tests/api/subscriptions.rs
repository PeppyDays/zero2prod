use fake::faker::internet::en::SafeEmail;
use fake::faker::name::en::Name;
use fake::Fake;
use reqwest::header;
use reqwest::StatusCode;

use crate::helper::TestApp;

#[rstest::rstest]
#[case(name(), email())]
#[tokio::test]
async fn subscription_returns_status_200_with_valid_form_data(
    #[case] name: Option<String>,
    #[case] email: Option<String>,
) {
    // Arrange
    let app = TestApp::new().await;
    let body = generate_request_body(name.clone(), email.clone());

    // Act
    let response = app
        .http_client
        .post(app.get_server_request_url("/subscriptions"))
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(response.status(), StatusCode::OK);

    let saved = sqlx::query!("select name, email from subscriptions",)
        .fetch_one(&app.database_pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.name, name.unwrap());
    assert_eq!(saved.email, email.unwrap());
}

#[rstest::rstest]
#[case(None, email())]
#[case(name(), None)]
#[case(None, None)]
#[tokio::test]
async fn subscription_returns_status_400_when_mandatory_field_is_missing(
    #[case] name: Option<String>,
    #[case] email: Option<String>,
) {
    // Arrange
    let app = TestApp::new().await;

    let body = generate_request_body(name, email);

    // Act
    let response = app
        .http_client
        .post(app.get_server_request_url("/subscriptions"))
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

fn name() -> Option<String> {
    Some(Name().fake())
}

fn email() -> Option<String> {
    Some(SafeEmail().fake())
}

fn generate_request_body(name: Option<String>, email: Option<String>) -> String {
    let mut body = String::new();
    if let Some(name) = name {
        body.push_str(format!("&name={}", &urlencoding::encode(name.as_str())).as_str());
    };
    if let Some(email) = email {
        body.push_str(format!("&email={}", &urlencoding::encode(email.as_str())).as_str());
    };
    body.trim_start_matches("&").to_string()
}
