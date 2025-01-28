use fake::faker::internet::en::SafeEmail;
use fake::faker::name::en::Name;
use fake::Fake;
use zero2prod::aggregates::subscriber::domain::service::Command;
use zero2prod::aggregates::subscriber::domain::service::SubscribeCommand;

#[rstest::fixture]
pub fn subscribe_command(name: String, email: String) -> Command {
    SubscribeCommand::new(name, email).into()
}

#[rstest::fixture]
pub fn name() -> String {
    Name().fake()
}

#[rstest::fixture]
pub fn email() -> String {
    SafeEmail().fake()
}
