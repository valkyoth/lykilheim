//! Audit interfaces.
//!
//! The foundation release defines the ownership boundary. Durable fail-closed
//! audit sinks are implemented in later releases.

use serde::Serialize;

/// Security-sensitive event emitted by the API path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AuditEvent {
    /// Event action, such as `sys.health`.
    pub action: String,
    /// Event path, such as `/v1/sys/health`.
    pub path: String,
}

/// Audit sink contract.
pub trait AuditSink: Send + Sync {
    /// Record one audit event.
    fn record(&self, event: &AuditEvent) -> crate::error::Result<()>;
}
