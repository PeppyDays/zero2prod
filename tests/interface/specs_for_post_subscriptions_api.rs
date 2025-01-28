use std::time::Duration;

use reqwest::StatusCode;
use wiremock::ResponseTemplate;

use crate::aggregates::subscriber::domain::command::email;
use crate::aggregates::subscriber::domain::command::name;
use crate::interface::system::system;
use crate::interface::system::System;

#[rstest::rstest]
#[tokio::test]
async fn subscription_returns_status_200_with_valid_form_data(
    #[future(awt)] system: System,
    name: String,
    email: String,
) {
    // Act
    let response = system
        .request
        .post_subscriptions(Some(name.clone()), Some(email.clone()))
        .await;

    // Assert
    assert_eq!(response.status(), StatusCode::OK);

    let pool = system.dependencies.subscriber_database_pool;
    let actual = sqlx::query!("select name, email from subscribers",)
        .fetch_one(&pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(actual.name, name);
    assert_eq!(actual.email, email);
}

#[rstest::rstest]
#[case(None, Some(email()))]
#[case(Some(name()), None)]
#[case(None, None)]
#[tokio::test]
async fn subscription_returns_status_400_when_mandatory_field_is_missing(
    #[future(awt)] system: System,
    #[case] name: Option<String>,
    #[case] email: Option<String>,
) {
    // Act
    let response = system.request.post_subscriptions(name, email).await;

    // Assert
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[rstest::rstest]
#[tokio::test]
async fn sut_returns_status_500_when_email_client_does_not_respond_in_3_seconds(
    #[future(awt)] system: System,
    name: String,
    email: String,
) {
    // Arrange
    system
        .dependencies
        .reset_subscription_email_server(Some(
            ResponseTemplate::new(StatusCode::OK).set_delay(Duration::from_secs(4)),
        ))
        .await;

    // Act
    let response = system
        .request
        .post_subscriptions(Some(name), Some(email))
        .await;

    // Assert
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}
