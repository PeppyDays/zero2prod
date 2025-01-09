use chrono::DateTime;
use chrono::Utc;
use uuid::ContextV7;
use uuid::Timestamp;
use uuid::Uuid;

pub struct Subscriber {
    pub id: Uuid,
    pub name: String,
    pub email: Email,
    pub subscribed_at: DateTime<Utc>,
}

impl Subscriber {
    pub fn new(name: String, email: Email) -> Self {
        Self {
            id: Uuid::new_v7(Timestamp::now(ContextV7::new())),
            name,
            email,
            subscribed_at: Utc::now(),
        }
    }
}

pub struct Email(String);

impl TryFrom<String> for Email {
    type Error = String;

    fn try_from(email: String) -> Result<Self, Self::Error> {
        if email.len() < 5 || !email.contains('@') {
            return Err("Invalid email".into());
        }
        Ok(Self(email))
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[async_trait::async_trait]
pub trait SubscriptionRepository: Send + Sync + 'static {
    async fn save(&self, subscriber: &Subscriber) -> Result<(), String>;
}
