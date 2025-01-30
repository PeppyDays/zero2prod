use std::time::Duration;

use reqwest::Client;
use secrecy::ExposeSecret;
use secrecy::SecretString;

use crate::aggregates::subscriber::domain::error::Error;
use crate::aggregates::subscriber::domain::infrastructure::EmailClient;
use crate::aggregates::subscriber::domain::model::Subscriber;

#[derive(Clone)]
pub struct FakeEmailClient {
    client: Client,
    host: String,
    sender: String,
    token: SecretString,
    timeout: Duration,
}

impl FakeEmailClient {
    pub fn new(
        client: Client,
        host: String,
        sender: String,
        token: SecretString,
        timeout: Duration,
    ) -> Self {
        Self {
            client,
            host,
            sender,
            token,
            timeout,
        }
    }
}

#[async_trait::async_trait]
impl EmailClient for FakeEmailClient {
    async fn send(
        &self,
        recipient: &Subscriber,
        subject: &str,
        content: &str,
    ) -> Result<(), Error> {
        let url = format!("{}/email", self.host);
        let body = SendEmailRequest {
            from: self.sender.as_ref(),
            to: recipient.email(),
            subject,
            content,
        };

        let _ = self
            .client
            .post(url)
            .header("X-Postmark-Server-Token", self.token.expose_secret())
            .json(&body)
            .timeout(self.timeout)
            .send()
            .await
            .map_err(|_e| Error::EmailOperationFailed)?
            .error_for_status()
            .map_err(|_e| Error::EmailOperationFailed)?;

        Ok(())
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "PascalCase")]
struct SendEmailRequest<'a> {
    from: &'a str,
    to: &'a str,
    subject: &'a str,
    content: &'a str,
}
