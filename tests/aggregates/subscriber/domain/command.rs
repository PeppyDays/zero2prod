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
pub fn subscribe_commands(#[default(5)] size: usize) -> Vec<Command> {
    (0..size)
        .map(|_| Command::from(SubscribeCommand::new(name(), email())))
        .collect()
}

#[rstest::fixture]
pub fn name() -> String {
    Name().fake()
}

#[rstest::fixture]
pub fn email() -> String {
    SafeEmail().fake()
}
