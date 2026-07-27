use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("validation failed: {0}")]
    Validation(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("storage error: {0}")]
    Storage(#[from] stark_storage::StorageError),
}

pub type Result<T> = std::result::Result<T, CommandError>;

/// Wire format for errors crossing into the frontend.
#[derive(Debug, Serialize)]
pub struct ErrorPayload {
    pub kind: String,
    pub message: String,
}

impl From<CommandError> for ErrorPayload {
    fn from(e: CommandError) -> Self {
        let kind = match &e {
            CommandError::Validation(_) => "VALIDATION",
            CommandError::NotFound(_) => "NOT_FOUND",
            CommandError::Storage(_) => "STORAGE",
        };
        ErrorPayload {
            kind: kind.to_string(),
            message: e.to_string(),
        }
    }
}