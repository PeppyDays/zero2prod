use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    InvariantViolated(String),
    #[error("Failed to find the token.")]
    TokenNotFound(String),
    #[error("Failed to find the subscriber.")]
    SubscriberNotFound(Uuid),
    #[error("Failed to operate on repository.")]
    RepositoryOperationFailed(#[source] anyhow::Error),
    #[error("Failed to process Email request.")]
    EmailOperationFailed(#[source] anyhow::Error),
    #[error("Failed unexpectedly.")]
    FailedUnexpectedly(#[source] anyhow::Error),
}
