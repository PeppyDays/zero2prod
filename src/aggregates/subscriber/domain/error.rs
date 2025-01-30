#[derive(Debug)]
pub enum Error {
    InvalidAttributes,
    MismatchedCommand,
    TokenNotFound,
    SubscriberNotFound,
    FailedRepositoryOperation,
    FailedEmailOperation,
    Unexpected,
}
