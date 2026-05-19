//! Project-wide error type using thiserror.

use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum OxideError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("regex error: {0}")]
    Regex(#[from] regex::Error),

    #[error("parse error on line {line}: {message}")]
    Parse { line: usize, message: String },

    #[error("config error: {0}")]
    Config(String),
}

#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, OxideError>;
