use fake::faker::internet::en::SafeEmail as FakeEmail;
use fake::faker::name::en::Name as FakeName;
use fake::Fake;
use uuid::Uuid;
use zero2prod::aggregates::subscriber::domain::model::Email;
use zero2prod::aggregates::subscriber::domain::model::Name;
use zero2prod::aggregates::subscriber::domain::model::Subscriber;
use zero2prod::aggregates::subscriber::domain::model::SubscriptionToken;
use zero2prod::aggregates::subscriber::infrastructure::repository::SubscriptionTokenDataModel;

#[rstest::fixture]
pub fn subscriber(name: Name, email: Email) -> Subscriber {
    Subscriber::create(name.as_ref(), email.as_ref()).unwrap()
}

#[rstest::fixture]
pub fn subscription_token(
    #[default(Uuid::now_v7().to_string())] token: String,
    #[default(Uuid::now_v7())] subscriber_id: Uuid,
) -> SubscriptionToken {
    SubscriptionTokenDataModel::new(token, subscriber_id)
        .try_into()
        .unwrap()
}

#[rstest::fixture]
pub fn name() -> Name {
    Name::parse(FakeName().fake::<String>().as_str()).unwrap()
}

#[rstest::fixture]
pub fn email() -> Email {
    Email::parse(FakeEmail().fake::<String>().as_str()).unwrap()
}

#[rstest::fixture]
pub fn token() -> String {
    Uuid::now_v7().into()
}
