use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Detection error: {0}")]
    DetectionError(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Adapter error: {0}")]
    AdapterError(String),

    #[error("Token counting error: {0}")]
    TokenCountError(String),

    #[error("Optimization error: {0}")]
    OptimizationError(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl Error {
    pub fn is_not_found(&self) -> bool {
        matches!(self, Error::FileNotFound(_))
    }
}
