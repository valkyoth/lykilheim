//! Cryptographic interfaces.
//!
//! Concrete key management and barrier encryption start in `0.2.0`.

use zeroize::{ZeroizeOnDrop, Zeroizing};

/// Marker trait for plaintext secret buffers that must be zeroized by concrete
/// implementations.
pub trait SecretBuffer: Send + Sync + ZeroizeOnDrop {
    /// Return the secret bytes for immediate cryptographic use.
    fn expose(&self) -> &[u8];
}

/// Barrier encryption contract.
pub trait Barrier: Send + Sync {
    /// Encrypt plaintext into authenticated ciphertext.
    fn encrypt(&self, plaintext: &[u8]) -> crate::error::Result<Vec<u8>>;

    /// Decrypt authenticated ciphertext into plaintext that is zeroized on drop.
    fn decrypt(&self, ciphertext: &[u8]) -> crate::error::Result<Zeroizing<Vec<u8>>>;
}
