//! Error and API error response types.

use std::{io, net::AddrParseError};

use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use thiserror::Error;

/// Crate-wide result type.
pub type Result<T> = std::result::Result<T, Error>;

/// Lykilheim application errors.
#[derive(Debug, Error)]
pub enum Error {
    /// Configuration is invalid.
    #[error("configuration error: {0}")]
    Config(String),

    /// I/O failed.
    #[error("io error: {0}")]
    Io(#[from] io::Error),

    /// Address parsing failed.
    #[error("address parse error: {0}")]
    AddrParse(#[from] AddrParseError),

    /// TOML parsing failed.
    #[error("toml decode error: {0}")]
    TomlDecode(#[from] toml::de::Error),

    /// HTTP server failed.
    #[error("http server error: {0}")]
    Http(#[from] axum::Error),

    /// A requested operation is intentionally not implemented yet.
    #[error("not implemented: {0}")]
    NotImplemented(&'static str),
}

/// Stable JSON error body returned by the HTTP API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ErrorBody {
    /// Machine-readable error code.
    pub code: &'static str,
    /// Human-readable error message.
    pub message: String,
}

impl Error {
    /// Convert this error into a status and stable JSON body.
    #[must_use]
    pub fn to_response_parts(&self) -> (StatusCode, ErrorBody) {
        match self {
            Self::Config(message) => (
                StatusCode::BAD_REQUEST,
                ErrorBody {
                    code: "configuration_error",
                    message: message.clone(),
                },
            ),
            Self::NotImplemented(_) => (
                StatusCode::NOT_IMPLEMENTED,
                ErrorBody {
                    code: "not_implemented",
                    message: "this endpoint is not available in this release".to_owned(),
                },
            ),
            Self::Io(_) | Self::AddrParse(_) | Self::TomlDecode(_) | Self::Http(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorBody {
                    code: "internal_error",
                    message: "internal server error".to_owned(),
                },
            ),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status, body) = self.to_response_parts();
        (status, Json(body)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;

    use super::Error;

    #[test]
    fn config_error_serializes_stable_body() {
        let error = Error::Config("listen address is required".to_owned());
        let (status, body) = error.to_response_parts();

        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(body.code, "configuration_error");
        assert_eq!(body.message, "listen address is required");
    }

    #[test]
    fn internal_errors_do_not_leak_details() {
        let error = Error::Io(std::io::Error::other("secret path failed"));
        let (status, body) = error.to_response_parts();

        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(body.code, "internal_error");
        assert_eq!(body.message, "internal server error");
    }

    #[test]
    fn not_implemented_errors_do_not_leak_roadmap() {
        let error = Error::NotImplemented("sys/init is implemented in a later version");
        let (status, body) = error.to_response_parts();

        assert_eq!(status, StatusCode::NOT_IMPLEMENTED);
        assert_eq!(body.code, "not_implemented");
        assert_eq!(
            body.message,
            "this endpoint is not available in this release"
        );
    }
}
