use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    Network(reqwest::Error),
    Serialization(serde_json::Error),
    ServerError(u16, String),
    InvalidConfig(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::Network(e) => write!(f, "Network error: {}", e),
            ApiError::Serialization(e) => write!(f, "Serialization error: {}", e),
            ApiError::ServerError(code, msg) => {
                write!(f, "Server error ({}): {}", code, msg)
            }
            ApiError::InvalidConfig(msg) => write!(f, "Invalid configuration: {}", msg),
        }
    }
}

impl std::error::Error for ApiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ApiError::Network(e) => Some(e),
            ApiError::Serialization(e) => Some(e),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        ApiError::Network(err)
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        ApiError::Serialization(err)
    }
}
