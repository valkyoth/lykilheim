//! HTTP API shape for the foundation release.

use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use axum::{
    Json, Router,
    body::{Body, Bytes},
    extract::{ConnectInfo, Request, State},
    http::{HeaderValue, StatusCode, header},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde::Serialize;
use tower_http::{limit::RequestBodyLimitLayer, set_header::SetResponseHeaderLayer};

use crate::{
    VERSION,
    config::Config,
    error::{Error, ErrorBody},
};

const REQUESTS_PER_SECOND: f64 = 100.0;
const RATE_LIMIT_CAPACITY: f64 = 100.0;
const RATE_LIMIT_STALE_AFTER: Duration = Duration::from_secs(60);

/// Shared HTTP API state.
#[derive(Debug, Clone)]
pub struct AppState {
    version: &'static str,
    initialized: bool,
    sealed: bool,
    rate_limit: Arc<RateLimitState>,
}

#[derive(Debug, Default)]
struct RateLimitState {
    buckets: Mutex<HashMap<IpAddr, TokenBucket>>,
}

#[derive(Debug)]
struct TokenBucket {
    tokens: f64,
    last_refill: Instant,
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
            rate_limit: Arc::new(RateLimitState::default()),
        }
    }
}

/// Build the versioned API router.
pub fn router(state: AppState) -> Router {
    let rate_limit_state = state.clone();

    Router::new()
        .route("/v1/sys/health", get(health))
        .route("/v1/sys/seal-status", get(seal_status))
        .route("/v1/sys/version", get(version))
        .route("/v1/sys/init", post(init))
        .route("/v1/sys/unseal", post(unseal))
        .route("/v1/sys/seal", post(seal))
        .layer(middleware::from_fn_with_state(
            rate_limit_state,
            enforce_rate_limit,
        ))
        .layer(RequestBodyLimitLayer::new(64 * 1024))
        .layer(SetResponseHeaderLayer::overriding(
            header::CACHE_CONTROL,
            HeaderValue::from_static("no-store"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::CONTENT_SECURITY_POLICY,
            HeaderValue::from_static("default-src 'none'; frame-ancestors 'none'"),
        ))
        .with_state(state)
}

async fn enforce_rate_limit(
    State(state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let ip = request
        .extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map_or(IpAddr::V4(Ipv4Addr::LOCALHOST), |connect_info| {
            connect_info.0.ip()
        });

    if !state.rate_limit.allow_one_request(ip) {
        return (
            StatusCode::TOO_MANY_REQUESTS,
            [(header::RETRY_AFTER, HeaderValue::from_static("1"))],
            Json(ErrorBody {
                code: "rate_limited",
                message: "too many requests".to_owned(),
            }),
        )
            .into_response();
    }

    next.run(request).await
}

impl RateLimitState {
    fn allow_one_request(&self, ip: IpAddr) -> bool {
        let now = Instant::now();
        let Ok(mut buckets) = self.buckets.lock() else {
            return false;
        };

        buckets
            .retain(|_, bucket| now.duration_since(bucket.last_refill) <= RATE_LIMIT_STALE_AFTER);

        buckets
            .entry(ip)
            .or_insert_with(|| TokenBucket::new(now))
            .allow(now)
    }
}

impl TokenBucket {
    fn new(now: Instant) -> Self {
        Self {
            tokens: RATE_LIMIT_CAPACITY,
            last_refill: now,
        }
    }

    fn allow(&mut self, now: Instant) -> bool {
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        self.tokens = (self.tokens + elapsed * REQUESTS_PER_SECOND).min(RATE_LIMIT_CAPACITY);
        self.last_refill = now;

        if self.tokens < 1.0 {
            return false;
        }

        self.tokens -= 1.0;
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct HealthResponse {
    initialized: bool,
    sealed: bool,
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

async fn init(_body: Bytes) -> Error {
    Error::NotImplemented("sys/init")
}

async fn unseal(_body: Bytes) -> Error {
    Error::NotImplemented("sys/unseal")
}

async fn seal(_body: Bytes) -> Error {
    Error::NotImplemented("sys/seal")
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
        assert_eq!(
            response_header(&response, "cache-control"),
            Some("no-store")
        );
        assert_eq!(
            response_header(&response, "x-content-type-options"),
            Some("nosniff")
        );

        let body = to_json(response.into_body()).await?;
        assert_eq!(body["initialized"], false);
        assert_eq!(body["sealed"], true);
        assert!(body.get("version").is_none());
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
        assert_eq!(
            body["message"],
            "this endpoint is not available in this release"
        );
        Ok(())
    }

    #[tokio::test]
    async fn large_request_body_is_rejected() -> Result<(), Box<dyn std::error::Error>> {
        let body = Body::from(vec![0_u8; 64 * 1024 + 1]);
        let mut request = Request::new(body);
        *request.method_mut() = "POST".parse()?;
        *request.uri_mut() = "/v1/sys/init".parse::<Uri>()?;

        let response = router(AppState::foundation()).oneshot(request).await?;

        assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
        Ok(())
    }

    #[tokio::test]
    async fn rate_limit_returns_retry_after() -> Result<(), Box<dyn std::error::Error>> {
        let app = router(AppState::foundation());
        let mut response = app
            .clone()
            .oneshot(request("GET", "/v1/sys/health")?)
            .await?;

        for _ in 0..100 {
            response = app
                .clone()
                .oneshot(request("GET", "/v1/sys/health")?)
                .await?;
        }

        assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
        assert_eq!(response_header(&response, "retry-after"), Some("1"));
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

    fn response_header<'a>(response: &'a axum::response::Response, name: &str) -> Option<&'a str> {
        response.headers().get(name)?.to_str().ok()
    }
}
