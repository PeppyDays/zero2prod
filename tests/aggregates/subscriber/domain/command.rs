use zero2prod::aggregates::subscriber::domain::model::Email;
use zero2prod::aggregates::subscriber::domain::model::Name;
use zero2prod::aggregates::subscriber::domain::service::Command;
use zero2prod::aggregates::subscriber::domain::service::ConfirmSubscriptionCommand;
use zero2prod::aggregates::subscriber::domain::service::SubscribeCommand;

use crate::aggregates::subscriber::domain::model::email;
use crate::aggregates::subscriber::domain::model::name;
use crate::aggregates::subscriber::domain::model::token;

#[rstest::fixture]
pub fn subscribe_command(name: Name, email: Email) -> Command {
    SubscribeCommand::new(name.as_ref().into(), email.as_ref().into()).into()
}

#[rstest::fixture]
pub fn subscribe_commands(#[default(5)] size: usize) -> Vec<Command> {
    (0..size)
        .map(|_| {
            Command::from(SubscribeCommand::new(
                name().as_ref().into(),
                email().as_ref().into(),
            ))
        })
        .collect()
}

#[rstest::fixture]
pub fn confirm_subscription_command(token: String) -> Command {
    ConfirmSubscriptionCommand::new(token).into()
}
