#[derive(Debug)]
pub enum Error {
    InvalidAttributes,
    FailedRepositoryOperation,
    UnmatchedCommand,
    Unexpected,
}
