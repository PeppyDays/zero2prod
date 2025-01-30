#[derive(Debug)]
pub enum Error {
    InvalidAttribute,
    CommandMismatched,
    TokenNotFound,
    SubscriberNotFound,
    RepositoryOperationFailed,
    EmailOperationFailed,
    FailedUnexpectedly,
}
