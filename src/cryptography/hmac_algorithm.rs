use hmac::{Hmac, Mac};
use sha2::{Sha256, Sha512};
use sha3::{Sha3_256, Sha3_512};
use anyhow::Result;

/// HMAC algorithm variants
/// Note: Blake2 has built-in keyed hashing which is better than HMAC,
/// so it's not included here. Use Blake2 directly for keyed hashing.
pub enum HmacAlgorithm {
    HmacSha256,
    HmacSha512,
    HmacSha3_256,
    HmacSha3_512,
}

impl HmacAlgorithm {
    /// Compute HMAC for the given key and message
    pub fn compute_hmac(&self, key: &[u8], message: &[u8]) -> Result<String> {
        match self {
            HmacAlgorithm::HmacSha256 => {
                type HmacSha256 = Hmac<Sha256>;
                let mut mac = HmacSha256::new_from_slice(key)
                    .map_err(|e| anyhow::Error::msg(format!("Invalid key length: {}", e)))?;
                mac.update(message);
                let result = mac.finalize();
                Ok(hex::encode(result.into_bytes()))
            }
            HmacAlgorithm::HmacSha512 => {
                type HmacSha512 = Hmac<Sha512>;
                let mut mac = HmacSha512::new_from_slice(key)
                    .map_err(|e| anyhow::Error::msg(format!("Invalid key length: {}", e)))?;
                mac.update(message);
                let result = mac.finalize();
                Ok(hex::encode(result.into_bytes()))
            }
            HmacAlgorithm::HmacSha3_256 => {
                type HmacSha3_256 = Hmac<Sha3_256>;
                let mut mac = HmacSha3_256::new_from_slice(key)
                    .map_err(|e| anyhow::Error::msg(format!("Invalid key length: {}", e)))?;
                mac.update(message);
                let result = mac.finalize();
                Ok(hex::encode(result.into_bytes()))
            }
            HmacAlgorithm::HmacSha3_512 => {
                type HmacSha3_512 = Hmac<Sha3_512>;
                let mut mac = HmacSha3_512::new_from_slice(key)
                    .map_err(|e| anyhow::Error::msg(format!("Invalid key length: {}", e)))?;
                mac.update(message);
                let result = mac.finalize();
                Ok(hex::encode(result.into_bytes()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hmac_sha256() {
        let key = b"secret_key";
        let message = b"test message";
        let result = HmacAlgorithm::HmacSha256.compute_hmac(key, message);
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn test_hmac_sha512() {
        let key = b"secret_key";
        let message = b"test message";
        let result = HmacAlgorithm::HmacSha512.compute_hmac(key, message);
        assert!(result.is_ok());
        let hash = result.unwrap();
        assert_eq!(hash.len(), 128); // SHA-512 produces 64 bytes = 128 hex chars
    }

    #[test]
    fn test_hmac_sha3_256() {
        let key = b"secret_key";
        let message = b"test message";
        let result = HmacAlgorithm::HmacSha3_256.compute_hmac(key, message);
        assert!(result.is_ok());
        let hash = result.unwrap();
        assert_eq!(hash.len(), 64); // SHA3-256 produces 32 bytes = 64 hex chars
    }

    #[test]
    fn test_hmac_consistency() {
        let key = b"consistent_key";
        let message = b"consistent message";

        let result1 = HmacAlgorithm::HmacSha256.compute_hmac(key, message).unwrap();
        let result2 = HmacAlgorithm::HmacSha256.compute_hmac(key, message).unwrap();

        assert_eq!(result1, result2, "HMAC should produce consistent results");
    }
}
