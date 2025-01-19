use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;
use validator::ValidateEmail;

use crate::subscription::exception::Error;

#[derive(Clone)]
pub struct Subscriber {
    id: Uuid,
    name: Name,
    email: Email,
    subscribed_at: DateTime<Utc>,
}

impl Subscriber {
    pub fn create(name: &str, email: &str) -> Result<Self, Error> {
        let name: Name = name.try_into().map_err(|_| Error::InvalidAttributes)?;
        let email: Email = email.try_into().map_err(|_| Error::InvalidAttributes)?;

        Ok(Self {
            id: Uuid::now_v7(),
            name,
            email,
            subscribed_at: Utc::now(),
        })
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
    fn parse(name: &str) -> Result<Self, Error> {
        if name.trim().is_empty() {
            return Err(Error::InvalidAttributes);
        }

        if name.len() >= 256 {
            return Err(Error::InvalidAttributes);
        }

        if name.chars().any(|c| FORBIDDEN_CHARACTERS.contains(&c)) {
            return Err(Error::InvalidAttributes);
        }

        Ok(Name(name.into()))
    }
}

impl TryFrom<&str> for Name {
    type Error = Error;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        Name::parse(name)
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
    fn parse(email: &str) -> Result<Self, Error> {
        email
            .validate_email()
            .then_some(Email(email.into()))
            .ok_or(Error::InvalidAttributes)
    }
}

impl TryFrom<&str> for Email {
    type Error = Error;

    fn try_from(email: &str) -> Result<Self, Self::Error> {
        Email::parse(email)
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
