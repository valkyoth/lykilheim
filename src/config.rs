//! Configuration loading and validation.

use std::{fs, net::SocketAddr, path::Path};

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

/// Top-level Lykilheim configuration.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    /// Server listener settings.
    pub server: ServerConfig,
    /// Audit behavior settings.
    pub audit: AuditConfig,
}

/// HTTP server configuration.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ServerConfig {
    /// TCP listen address for the API server.
    pub listen: SocketAddr,
}

/// Audit configuration placeholder for the foundation release.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
#[serde(default, deny_unknown_fields)]
pub struct AuditConfig {
    /// Whether audit is required for mutating operations.
    ///
    /// This defaults to `false` in `0.1.0` because no durable audit sink exists
    /// yet. Later releases must fail closed when this is enabled without a sink.
    pub required: bool,
}

impl Config {
    /// Load configuration from an optional TOML path.
    pub fn load_optional(path: Option<&Path>) -> Result<Self> {
        match path {
            Some(path) => Self::from_file(path),
            None => Ok(Self::default()),
        }
    }

    /// Load configuration from a TOML file.
    pub fn from_file(path: &Path) -> Result<Self> {
        let source = fs::read_to_string(path)?;
        Self::from_toml_str(&source)
    }

    /// Parse configuration from TOML text.
    pub fn from_toml_str(source: &str) -> Result<Self> {
        let config: Self = toml::from_str(source)?;
        config.validate()?;
        Ok(config)
    }

    /// Validate security-sensitive configuration invariants.
    pub fn validate(&self) -> Result<()> {
        if self.server.listen.port() == 0 {
            return Err(Error::Config(
                "server.listen must use a non-zero TCP port".to_owned(),
            ));
        }
        if self.audit.required {
            return Err(Error::Config(
                "audit.required cannot be true in 0.1.0 because no durable audit sink is available"
                    .to_owned(),
            ));
        }
        Ok(())
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            listen: SocketAddr::from(([127, 0, 0, 1], 8200)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;

    use crate::error::Result;

    use super::Config;

    #[test]
    fn default_config_is_local_and_audit_honest() {
        let config = Config::default();

        assert_eq!(
            config.server.listen,
            SocketAddr::from(([127, 0, 0, 1], 8200))
        );
        assert!(!config.audit.required);
    }

    #[test]
    fn parses_toml_config() -> Result<()> {
        let config = Config::from_toml_str(
            r#"
            [server]
            listen = "127.0.0.1:18200"

            [audit]
            required = false
            "#,
        )?;

        assert_eq!(
            config.server.listen,
            SocketAddr::from(([127, 0, 0, 1], 18200))
        );
        assert!(!config.audit.required);
        Ok(())
    }

    #[test]
    fn rejects_zero_port() -> Result<()> {
        let Err(error) = Config::from_toml_str(
            r#"
            [server]
            listen = "127.0.0.1:0"
            "#,
        ) else {
            return Err(crate::error::Error::Config(
                "zero listen port should fail".to_owned(),
            ));
        };

        assert!(
            error
                .to_string()
                .contains("server.listen must use a non-zero TCP port")
        );
        Ok(())
    }

    #[test]
    fn rejects_required_audit_without_sink() -> Result<()> {
        let Err(error) = Config::from_toml_str(
            r#"
            [audit]
            required = true
            "#,
        ) else {
            return Err(crate::error::Error::Config(
                "required audit should fail before durable sinks exist".to_owned(),
            ));
        };

        assert!(
            error
                .to_string()
                .contains("audit.required cannot be true in 0.1.0")
        );
        Ok(())
    }
}
