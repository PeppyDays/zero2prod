use chrono::DateTime;
use chrono::Utc;
use strum::AsRefStr;
use strum::EnumString;
use uuid::Uuid;
use validator::ValidateEmail;

use crate::subscriber::domain::error::Error;

#[derive(Clone, Debug)]
pub struct Subscriber {
    id: Uuid,
    name: Name,
    email: Email,
    subscribed_at: DateTime<Utc>,
    status: Status,
}

impl Subscriber {
    pub(crate) fn new(
        id: Uuid,
        name: Name,
        email: Email,
        subscribed_at: DateTime<Utc>,
        status: Status,
    ) -> Self {
        Self {
            id,
            name,
            email,
            subscribed_at,
            status,
        }
    }

    pub fn create(name: &str, email: &str) -> Result<Self, Error> {
        let name: Name = name.try_into()?;
        let email: Email = email.try_into()?;

        Ok(Self {
            id: Uuid::now_v7(),
            name,
            email,
            subscribed_at: Utc::now(),
            status: Status::Pending,
        })
    }

    pub fn confirm(&mut self) {
        self.status = Status::Confirmed;
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

    pub fn status(&self) -> &Status {
        &self.status
    }
}

const FORBIDDEN_CHARACTERS: [char; 11] = ['/', '(', ')', '\"', '<', '>', '\\', '{', '}', '?', '%'];

#[derive(Clone, Debug)]
pub struct Name(String);

impl Name {
    /// .
    ///
    /// # Safety
    /// This constructor creates Name without validation. Be careful to use it.
    /// For now, this is only for repository to create Name without validation.
    /// .
    pub unsafe fn new_unchecked(name: &str) -> Self {
        Self(name.into())
    }

    pub fn parse(name: &str) -> Result<Self, Error> {
        if name.trim().is_empty() {
            return Err(Error::InvariantViolated("Name cannot be empty".into()));
        }

        if name.len() >= 256 {
            return Err(Error::InvariantViolated(
                "Name cannot be longer than 256".into(),
            ));
        }

        if name.chars().any(|c| FORBIDDEN_CHARACTERS.contains(&c)) {
            return Err(Error::InvariantViolated(
                "Name cannot have forbidden characters".into(),
            ));
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

#[derive(Clone, Debug)]
pub struct Email(String);

impl Email {
    /// .
    ///
    /// # Safety
    /// This constructor creates Email without validation. Be careful to use it.
    /// For now, this is only for repository to create Email without validation.
    /// .
    pub unsafe fn new_unchecked(email: &str) -> Self {
        Self(email.into())
    }

    pub fn parse(email: &str) -> Result<Self, Error> {
        email
            .validate_email()
            .then_some(Email(email.into()))
            .ok_or(Error::InvariantViolated(
                "Email address must be valid".into(),
            ))
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

#[derive(Clone, Debug, EnumString, AsRefStr)]
pub enum Status {
    Unexpected,
    Pending,
    Confirmed,
}

#[derive(Clone, Debug)]
pub struct SubscriptionToken {
    token: String,
    subscriber_id: Uuid,
}

impl SubscriptionToken {
    pub(crate) fn new(token: String, subscriber_id: Uuid) -> Self {
        Self {
            token,
            subscriber_id,
        }
    }

    pub fn create(subscriber_id: Uuid) -> Self {
        Self {
            subscriber_id,
            token: Uuid::now_v7().into(),
        }
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn subscriber_id(&self) -> &Uuid {
        &self.subscriber_id
    }
}

#[cfg(test)]
mod tests {
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    use super::*;

    #[derive(Clone, Debug)]
    struct ValidEmailFixture(String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let mut rng = StdRng::seed_from_u64(u64::arbitrary(g));
            let email = SafeEmail().fake_with_rng(&mut rng);
            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_correctly(email: ValidEmailFixture) -> bool {
        dbg!(&email.0);
        Email::parse(email.0.as_str()).is_ok()
    }
}
