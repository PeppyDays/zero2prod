#[derive(Debug)]
pub enum Error {
    InvalidAttributes,
    FailedRepositoryOperation,
    FailedEmailOperation,
    MismatchedCommand,
    Unexpected,
}
