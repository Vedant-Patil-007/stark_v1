#[derive(Debug, thiserror::Error)]
pub enum AiError {
    #[error("could not parse model output: {0}")]
    Parse(String),

    #[error("no interpretation found for input")]
    NoMatch,

    #[error("provider unavailable: {0}")]
    Unavailable(String),

    #[error("provider error: {0}")]
    Provider(String),
}

pub type Result<T> = std::result::Result<T, AiError>;