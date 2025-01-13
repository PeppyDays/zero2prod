use fake::faker::internet::en::SafeEmail;
use fake::faker::name::en::Name;
use fake::Fake;
use zero2prod::subscription::domain::service::Command;

#[rstest::fixture]
pub fn subscribe_command() -> Command {
    Command::Subscribe {
        name: Name().fake(),
        email: SafeEmail().fake(),
    }
}
