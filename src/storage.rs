//! Storage interfaces.
//!
//! Storage backends must only see opaque encrypted bytes once the barrier is
//! implemented.

/// Storage key namespace.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StorageKey(String);

impl StorageKey {
    /// Create a storage key after basic path validation.
    pub fn new(value: impl Into<String>) -> crate::error::Result<Self> {
        let value = value.into();
        if value.is_empty()
            || value.len() > 512
            || value.starts_with('/')
            || value.starts_with("./")
            || value.contains("..")
            || value.contains('\0')
        {
            return Err(crate::error::Error::Config(
                "storage keys must be clean relative paths without null bytes, '..', './' prefixes, or more than 512 bytes".to_owned(),
            ));
        }
        Ok(Self(value))
    }

    /// Return the key as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Opaque storage backend contract.
pub trait Storage: Send + Sync {
    /// Get encrypted bytes by key.
    fn get(&self, key: &StorageKey) -> crate::error::Result<Option<Vec<u8>>>;

    /// Put encrypted bytes by key.
    fn put(&self, key: &StorageKey, value: &[u8]) -> crate::error::Result<()>;

    /// Delete encrypted bytes by key.
    fn delete(&self, key: &StorageKey) -> crate::error::Result<()>;

    /// List encrypted keys by prefix.
    fn list(&self, prefix: &StorageKey) -> crate::error::Result<Vec<StorageKey>>;
}

#[cfg(test)]
mod tests {
    use super::StorageKey;

    #[test]
    fn storage_key_rejects_absolute_paths() {
        assert!(StorageKey::new("/sys/raw").is_err());
    }

    #[test]
    fn storage_key_rejects_parent_segments() {
        assert!(StorageKey::new("secret/../root").is_err());
    }

    #[test]
    fn storage_key_rejects_leading_current_directory() {
        assert!(StorageKey::new("./secret").is_err());
    }

    #[test]
    fn storage_key_rejects_null_bytes() {
        assert!(StorageKey::new("secret\0tail").is_err());
    }

    #[test]
    fn storage_key_rejects_long_values() {
        assert!(StorageKey::new("a".repeat(513)).is_err());
    }
}
