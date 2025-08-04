use anyhow::{Error, Result};
use rand::RngCore;
use rsa::{RsaPrivateKey, RsaPublicKey};
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey};
use base64::{Engine as _, engine::general_purpose::STANDARD};

pub enum JwtAlgorithm {
    HS256,
    HS384,
    HS512,
    RS256,
    RS384,
    RS512,
}

impl JwtAlgorithm {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_uppercase().as_str() {
            "HS256" => Ok(JwtAlgorithm::HS256),
            "HS384" => Ok(JwtAlgorithm::HS384),
            "HS512" => Ok(JwtAlgorithm::HS512),
            "RS256" => Ok(JwtAlgorithm::RS256),
            "RS384" => Ok(JwtAlgorithm::RS384),
            "RS512" => Ok(JwtAlgorithm::RS512),
            _ => Err(Error::msg(format!("Unsupported JWT algorithm: {}", s))),
        }
    }

    pub fn default_key_size(&self) -> usize {
        match self {
            JwtAlgorithm::HS256 => 32, // 256 bits
            JwtAlgorithm::HS384 => 48, // 384 bits
            JwtAlgorithm::HS512 => 64, // 512 bits
            JwtAlgorithm::RS256 | JwtAlgorithm::RS384 | JwtAlgorithm::RS512 => 2048, // RSA key size in bits
        }
    }

    pub fn is_symmetric(&self) -> bool {
        matches!(self, JwtAlgorithm::HS256 | JwtAlgorithm::HS384 | JwtAlgorithm::HS512)
    }
}

pub enum OutputFormat {
    Base64,
    Hex,
    Raw,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "base64" => Ok(OutputFormat::Base64),
            "hex" => Ok(OutputFormat::Hex),
            "raw" => Ok(OutputFormat::Raw),
            _ => Err(Error::msg(format!("Unsupported output format: {}", s))),
        }
    }
}

pub struct JwtKeyPair {
    pub private_key: String,
    pub public_key: Option<String>,
}

pub fn generate_jwt_secret(algorithm: JwtAlgorithm, format: OutputFormat, key_size: Option<usize>) -> Result<JwtKeyPair> {
    if algorithm.is_symmetric() {
        generate_symmetric_key(algorithm, format, key_size)
    } else {
        generate_asymmetric_keys(algorithm, format, key_size)
    }
}

fn generate_symmetric_key(algorithm: JwtAlgorithm, format: OutputFormat, key_size: Option<usize>) -> Result<JwtKeyPair> {
    let size = key_size.unwrap_or(algorithm.default_key_size());
    let mut key = vec![0u8; size];
    
    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut key);

    let formatted_key = match format {
        OutputFormat::Base64 => STANDARD.encode(&key),
        OutputFormat::Hex => hex::encode(&key),
        OutputFormat::Raw => String::from_utf8(key).map_err(|_| Error::msg("Invalid UTF-8 in raw key"))?,
    };

    Ok(JwtKeyPair {
        private_key: formatted_key,
        public_key: None,
    })
}

fn generate_asymmetric_keys(algorithm: JwtAlgorithm, _format: OutputFormat, key_size: Option<usize>) -> Result<JwtKeyPair> {
    let bits = key_size.unwrap_or(algorithm.default_key_size());
    let mut rng = rand::thread_rng();
    
    let private_key = RsaPrivateKey::new(&mut rng, bits)
        .map_err(|e| Error::msg(format!("Failed to generate RSA private key: {}", e)))?;
    
    let public_key = RsaPublicKey::from(&private_key);

    let private_pem = private_key.to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
        .map_err(|e| Error::msg(format!("Failed to encode private key to PEM: {}", e)))?;
    
    let public_pem = public_key.to_public_key_pem(rsa::pkcs8::LineEnding::LF)
        .map_err(|e| Error::msg(format!("Failed to encode public key to PEM: {}", e)))?;

    Ok(JwtKeyPair {
        private_key: private_pem.to_string(),
        public_key: Some(public_pem),
    })
}

pub fn save_key_to_file(key: &str, file_path: &str) -> Result<()> {
    use std::fs;
    fs::write(file_path, key)
        .map_err(|e| Error::msg(format!("Failed to write key to file {}: {}", file_path, e)))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hs256_key_generation() {
        let result = generate_jwt_secret(JwtAlgorithm::HS256, OutputFormat::Base64, None);
        assert!(result.is_ok());
        let key_pair = result.unwrap();
        assert!(key_pair.public_key.is_none());
        assert!(!key_pair.private_key.is_empty());
    }

    #[test]
    fn test_hs384_key_generation() {
        let result = generate_jwt_secret(JwtAlgorithm::HS384, OutputFormat::Hex, None);
        assert!(result.is_ok());
        let key_pair = result.unwrap();
        assert!(key_pair.public_key.is_none());
        assert_eq!(key_pair.private_key.len(), 96); // 48 bytes * 2 (hex encoding)
    }

    #[test]
    fn test_hs512_key_generation() {
        let result = generate_jwt_secret(JwtAlgorithm::HS512, OutputFormat::Hex, None);
        assert!(result.is_ok());
        let key_pair = result.unwrap();
        assert!(key_pair.public_key.is_none());
        assert_eq!(key_pair.private_key.len(), 128); // 64 bytes * 2 (hex encoding)
    }

    #[test]
    fn test_rs256_key_generation() {
        let result = generate_jwt_secret(JwtAlgorithm::RS256, OutputFormat::Base64, Some(1024));
        assert!(result.is_ok());
        let key_pair = result.unwrap();
        assert!(key_pair.public_key.is_some());
        assert!(!key_pair.private_key.is_empty());
        assert!(key_pair.private_key.contains("BEGIN PRIVATE KEY"));
        assert!(key_pair.public_key.unwrap().contains("BEGIN PUBLIC KEY"));
    }

    #[test]
    fn test_algorithm_from_string() {
        assert!(matches!(JwtAlgorithm::from_str("HS256"), Ok(JwtAlgorithm::HS256)));
        assert!(matches!(JwtAlgorithm::from_str("hs384"), Ok(JwtAlgorithm::HS384)));
        assert!(matches!(JwtAlgorithm::from_str("RS512"), Ok(JwtAlgorithm::RS512)));
        assert!(JwtAlgorithm::from_str("INVALID").is_err());
    }

    #[test]
    fn test_output_format_from_string() {
        assert!(matches!(OutputFormat::from_str("base64"), Ok(OutputFormat::Base64)));
        assert!(matches!(OutputFormat::from_str("HEX"), Ok(OutputFormat::Hex)));
        assert!(matches!(OutputFormat::from_str("raw"), Ok(OutputFormat::Raw)));
        assert!(OutputFormat::from_str("invalid").is_err());
    }
}