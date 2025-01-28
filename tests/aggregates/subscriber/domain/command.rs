use fake::faker::internet::en::SafeEmail;
use fake::faker::name::en::Name;
use fake::Fake;
use zero2prod::aggregates::subscriber::domain::service::command::executors::subscribe::Command as SubscribeCommand;
use zero2prod::aggregates::subscriber::domain::service::command::interface::Command;

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
