//! HTTP API shape for the foundation release.

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use serde::Serialize;

use crate::{VERSION, config::Config, error::Error};

/// Shared HTTP API state.
#[derive(Debug, Clone)]
pub struct AppState {
    version: &'static str,
    initialized: bool,
    sealed: bool,
}

impl AppState {
    /// Build API state from loaded configuration.
    #[must_use]
    pub fn from_config(_config: Config) -> Self {
        Self::foundation()
    }

    /// Build default foundation state.
    #[must_use]
    pub fn foundation() -> Self {
        Self {
            version: VERSION,
            initialized: false,
            sealed: true,
        }
    }
}

/// Build the versioned API router.
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/v1/sys/health", get(health))
        .route("/v1/sys/seal-status", get(seal_status))
        .route("/v1/sys/version", get(version))
        .route("/v1/sys/init", post(init))
        .route("/v1/sys/unseal", post(unseal))
        .route("/v1/sys/seal", post(seal))
        .with_state(state)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct HealthResponse {
    initialized: bool,
    sealed: bool,
    version: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct SealStatusResponse {
    initialized: bool,
    sealed: bool,
    threshold: u8,
    progress: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct VersionResponse {
    version: &'static str,
}

async fn health(State(state): State<AppState>) -> (StatusCode, Json<HealthResponse>) {
    (
        StatusCode::OK,
        Json(HealthResponse {
            initialized: state.initialized,
            sealed: state.sealed,
            version: state.version,
        }),
    )
}

async fn seal_status(State(state): State<AppState>) -> Json<SealStatusResponse> {
    Json(SealStatusResponse {
        initialized: state.initialized,
        sealed: state.sealed,
        threshold: 0,
        progress: 0,
    })
}

async fn version(State(state): State<AppState>) -> Json<VersionResponse> {
    Json(VersionResponse {
        version: state.version,
    })
}

async fn init() -> Error {
    Error::NotImplemented("sys/init is defined for 0.1.0 but implemented in 0.2.0")
}

async fn unseal() -> Error {
    Error::NotImplemented("sys/unseal is defined for 0.1.0 but implemented in 0.2.0")
}

async fn seal() -> Error {
    Error::NotImplemented("sys/seal is defined for 0.1.0 but implemented in 0.2.0")
}

#[cfg(test)]
mod tests {
    use axum::{
        body::{Body, to_bytes},
        http::{Request, StatusCode, Uri},
    };
    use serde_json::Value;
    use tower::ServiceExt;

    use super::{AppState, router};

    #[tokio::test]
    async fn health_endpoint_returns_foundation_state() -> Result<(), Box<dyn std::error::Error>> {
        let response = router(AppState::foundation())
            .oneshot(request("GET", "/v1/sys/health")?)
            .await?;

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_json(response.into_body()).await?;
        assert_eq!(body["initialized"], false);
        assert_eq!(body["sealed"], true);
        assert_eq!(body["version"], env!("CARGO_PKG_VERSION"));
        Ok(())
    }

    #[tokio::test]
    async fn version_endpoint_returns_crate_version() -> Result<(), Box<dyn std::error::Error>> {
        let response = router(AppState::foundation())
            .oneshot(request("GET", "/v1/sys/version")?)
            .await?;

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_json(response.into_body()).await?;
        assert_eq!(body["version"], env!("CARGO_PKG_VERSION"));
        Ok(())
    }

    #[tokio::test]
    async fn init_endpoint_is_defined_but_not_implemented_yet()
    -> Result<(), Box<dyn std::error::Error>> {
        let response = router(AppState::foundation())
            .oneshot(request("POST", "/v1/sys/init")?)
            .await?;

        assert_eq!(response.status(), StatusCode::NOT_IMPLEMENTED);

        let body = to_json(response.into_body()).await?;
        assert_eq!(body["code"], "not_implemented");
        Ok(())
    }

    fn request(method: &str, path: &str) -> Result<Request<Body>, Box<dyn std::error::Error>> {
        let mut request = Request::new(Body::empty());
        *request.method_mut() = method.parse()?;
        *request.uri_mut() = path.parse::<Uri>()?;
        Ok(request)
    }

    async fn to_json(body: Body) -> Result<Value, Box<dyn std::error::Error>> {
        let bytes = to_bytes(body, 1024 * 1024).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }
}
