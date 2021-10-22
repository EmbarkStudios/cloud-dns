use std::fmt;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DnsError {
    #[error(transparent)]
    Dns { source: CloudDnsError },
    #[error(transparent)]
    Auth(#[from] tame_oauth::Error),
    #[error(transparent)]
    Json(#[from] serde_path_to_error::Error<serde_json::Error>),
    #[error(transparent)]
    Url(#[from] url::ParseError),
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    #[error(transparent)]
    Other {
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

/// An error returned from Cloud DNS's API.
#[derive(serde::Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct CloudDnsError {
    pub errors: Option<Vec<serde_json::Value>>,
    pub message: String,
}

impl fmt::Display for CloudDnsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.message)?;

        Ok(())
    }
}

impl std::error::Error for CloudDnsError {}
