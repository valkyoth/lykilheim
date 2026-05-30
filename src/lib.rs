//! Lykilheim library crate.
//!
//! The `0.1.0` foundation keeps the public API shape, configuration parsing,
//! and security-sensitive ownership boundaries small and explicit.

pub mod api;
pub mod audit;
pub mod config;
pub mod crypto;
pub mod error;
pub mod storage;

#[cfg(test)]
pub mod test_support;

/// Current crate version as declared by Cargo.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
