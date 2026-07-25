use thiserror::Error;

/// Error type returned by every fallible operation in the SDK.
#[derive(Debug, Error)]
pub enum RobinhoodChainError {
    /// API key was missing or malformed at construction time.
    ///
    /// Get a free key at <https://madeonsol.com/pricing> — Robinhood Chain
    /// coverage is bundled into every tier at no extra cost.
    #[error(
        "RobinhoodChain: apiKey is required and must start with `msk_`. \
         Get a free key at https://madeonsol.com/pricing"
    )]
    MissingApiKey,

    /// API returned a non-2xx HTTP status.
    #[error("Robinhood Chain API error ({status}): {message}")]
    Api {
        status: u16,
        message: String,
        body: serde_json::Value,
    },

    /// `reqwest` transport error (DNS, TLS, connection reset, etc).
    #[error("Robinhood Chain transport error: {0}")]
    Transport(#[from] reqwest::Error),

    /// JSON serialization or deserialization error.
    #[error("Robinhood Chain JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// URL-building error (only fires for impossible parameter combinations).
    #[error("Robinhood Chain URL error: {0}")]
    Url(#[from] url::ParseError),
}

impl RobinhoodChainError {
    /// Returns the HTTP status code if this is an [`RobinhoodChainError::Api`] variant.
    pub fn status(&self) -> Option<u16> {
        match self {
            RobinhoodChainError::Api { status, .. } => Some(*status),
            _ => None,
        }
    }

    /// Returns the raw response body if this is an [`RobinhoodChainError::Api`] variant.
    pub fn body(&self) -> Option<&serde_json::Value> {
        match self {
            RobinhoodChainError::Api { body, .. } => Some(body),
            _ => None,
        }
    }
}

pub type Result<T> = std::result::Result<T, RobinhoodChainError>;
