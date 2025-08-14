use thiserror::Error;

#[derive(Debug, Error)]
pub enum MonkeyTyperError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Config parse error: {0}")]
    ConfigParse(#[from] serde_json::Error),
}
