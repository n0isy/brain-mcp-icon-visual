use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResolveError {
    #[error("file not found: {0} ({1})")]
    FileNotFound(String, String),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
}
