//! Test support helpers.

use crate::api::{AppState, router};

/// Build a test router with foundation state.
pub fn test_router() -> axum::Router {
    router(AppState::foundation())
}
