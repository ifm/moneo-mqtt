#[derive(Debug, thiserror::Error)]
pub enum TopicFromStrErr {
    #[error("Expected {0} tokens in Topic, found {1}")]
    UnexpectedNumberOfTokens(usize, usize),

    #[error("Failed to parse UUID")]
    UuidParsingError(#[from] uuid::Error),

    #[error("Failed to parse port")]
    FailedToParsePort(#[from] std::num::ParseIntError),
}
