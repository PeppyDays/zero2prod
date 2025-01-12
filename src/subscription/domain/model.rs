use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

use crate::subscription::exception::Error;

#[derive(Clone)]
pub struct Subscriber {
    id: Uuid,
    name: Name,
    email: Email,
    subscribed_at: DateTime<Utc>,
}

impl Subscriber {
    pub fn new(name: Name, email: Email) -> Self {
        Self {
            id: Uuid::now_v7(),
            name,
            email,
            subscribed_at: Utc::now(),
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn email(&self) -> &str {
        self.email.as_ref()
    }

    pub fn subscribed_at(&self) -> &DateTime<Utc> {
        &self.subscribed_at
    }
}

const FORBIDDEN_CHARACTERS: [char; 11] = ['/', '(', ')', '\"', '<', '>', '\\', '{', '}', '?', '%'];

#[derive(Clone)]
pub struct Name(String);

impl Name {
    fn validate(name: &str) -> Result<(), Error> {
        if name.trim().is_empty() {
            return Err(Error::InvalidAttributes);
        }

        if name.len() >= 256 {
            return Err(Error::InvalidAttributes);
        }

        if name.chars().any(|c| FORBIDDEN_CHARACTERS.contains(&c)) {
            return Err(Error::InvalidAttributes);
        }

        Ok(())
    }
}

impl TryFrom<String> for Name {
    type Error = Error;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        Name::validate(name.as_str()).map(|_| Name(name))
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Clone)]
pub struct Email(String);

impl Email {
    fn validate(email: &str) -> Result<(), Error> {
        if email.len() < 5 || !email.contains('@') {
            return Err(Error::InvalidAttributes);
        }
        Ok(())
    }
}

impl TryFrom<String> for Email {
    type Error = Error;

    fn try_from(email: String) -> Result<Self, Self::Error> {
        Email::validate(email.as_str()).map(|_| Email(email))
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
