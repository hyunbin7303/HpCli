use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key
};
use aes_gcm::aead::rand_core::RngCore;
use anyhow::Result;
use std::fs::File;
use std::io::{Read, Write};

const CHUNK_SIZE: usize = 128;  // Match XChaCha20 chunk size
const NONCE_SIZE: usize = 12;   // GCM standard nonce size
const SALT_SIZE: usize = 16;    // Salt for PBKDF2
const TAG_SIZE: usize = 16;     // GCM authentication tag size
const PBKDF2_ITERATIONS: u32 = 100_000;  // Secure iteration count

/// Encrypt a file using AES-256-GCM with password-based key derivation
pub fn encrypt_file_aes(
    input_path: &str,
    output_path: &str,
    password: &str
) -> Result<()> {
    // Read input file
    let plaintext = std::fs::read(input_path)?;

    // Generate random salt for PBKDF2
    let mut salt = [0u8; SALT_SIZE];
    OsRng.fill_bytes(&mut salt);

    // Derive 256-bit key from password using PBKDF2-HMAC-SHA256
    let mut key_bytes = [0u8; 32];
    pbkdf2::pbkdf2_hmac::<sha2::Sha256>(
        password.as_bytes(),
        &salt,
        PBKDF2_ITERATIONS,
        &mut key_bytes
    );

    // Generate random nonce
    let mut nonce_bytes = [0u8; NONCE_SIZE];
    OsRng.fill_bytes(&mut nonce_bytes);

    // Initialize AES-256-GCM cipher
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Create output file
    let mut output = File::create(output_path)?;

    // Write file format: [version][salt][nonce]
    output.write_all(&[0x02])?;  // Version byte for AES-256-GCM
    output.write_all(&salt)?;
    output.write_all(&nonce_bytes)?;

    // Encrypt data in chunks
    for chunk in plaintext.chunks(CHUNK_SIZE) {
        let ciphertext = cipher.encrypt(nonce, chunk)
            .map_err(|e| anyhow::Error::msg(format!("Encryption failed: {}", e)))?;
        output.write_all(&ciphertext)?;
    }

    Ok(())
}

/// Decrypt a file using AES-256-GCM with password-based key derivation
pub fn decrypt_file_aes(
    input_path: &str,
    output_path: &str,
    password: &str
) -> Result<()> {
    // Read encrypted file
    let encrypted_data = std::fs::read(input_path)?;

    // Parse header
    if encrypted_data.is_empty() || encrypted_data[0] != 0x02 {
        return Err(anyhow::Error::msg("Invalid file format or version"));
    }

    let header_size = 1 + SALT_SIZE + NONCE_SIZE;
    if encrypted_data.len() < header_size {
        return Err(anyhow::Error::msg("File too small to be valid AES-encrypted file"));
    }

    let salt = &encrypted_data[1..1 + SALT_SIZE];
    let nonce_bytes = &encrypted_data[1 + SALT_SIZE..header_size];
    let ciphertext = &encrypted_data[header_size..];

    // Derive key from password using same parameters
    let mut key_bytes = [0u8; 32];
    pbkdf2::pbkdf2_hmac::<sha2::Sha256>(
        password.as_bytes(),
        salt,
        PBKDF2_ITERATIONS,
        &mut key_bytes
    );

    // Initialize cipher
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce_bytes);

    // Decrypt data in chunks
    let mut plaintext = Vec::new();
    let chunk_with_tag_size = CHUNK_SIZE + TAG_SIZE;

    for chunk in ciphertext.chunks(chunk_with_tag_size) {
        let decrypted_chunk = cipher.decrypt(nonce, chunk)
            .map_err(|e| anyhow::Error::msg(format!("Decryption failed (wrong password or corrupted file): {}", e)))?;
        plaintext.extend_from_slice(&decrypted_chunk);
    }

    // Write decrypted data
    std::fs::write(output_path, plaintext)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_aes_encrypt_decrypt_roundtrip() {
        let dir = tempdir().unwrap();
        let input = dir.path().join("plain.txt");
        let encrypted = dir.path().join("encrypted.aes");
        let decrypted = dir.path().join("decrypted.txt");

        // Test data
        let test_data = b"This is a test file for AES-256-GCM encryption!";
        std::fs::write(&input, test_data).unwrap();

        let password = "secure_password_123";

        // Encrypt
        let encrypt_result = encrypt_file_aes(
            input.to_str().unwrap(),
            encrypted.to_str().unwrap(),
            password
        );
        assert!(encrypt_result.is_ok(), "Encryption should succeed");

        // Verify encrypted file exists and has correct version byte
        let encrypted_data = std::fs::read(&encrypted).unwrap();
        assert_eq!(encrypted_data[0], 0x02, "Version byte should be 0x02");
        assert!(encrypted_data.len() > test_data.len(), "Encrypted file should be larger");

        // Decrypt
        let decrypt_result = decrypt_file_aes(
            encrypted.to_str().unwrap(),
            decrypted.to_str().unwrap(),
            password
        );
        assert!(decrypt_result.is_ok(), "Decryption should succeed");

        // Verify content matches
        let decrypted_data = std::fs::read(&decrypted).unwrap();
        assert_eq!(decrypted_data, test_data, "Decrypted content should match original");

        dir.close().unwrap();
    }

    #[test]
    fn test_aes_wrong_password() {
        let dir = tempdir().unwrap();
        let input = dir.path().join("plain.txt");
        let encrypted = dir.path().join("encrypted.aes");
        let decrypted = dir.path().join("decrypted.txt");

        std::fs::write(&input, b"Secret data").unwrap();

        // Encrypt with one password
        encrypt_file_aes(
            input.to_str().unwrap(),
            encrypted.to_str().unwrap(),
            "correct_password"
        ).unwrap();

        // Try to decrypt with wrong password
        let result = decrypt_file_aes(
            encrypted.to_str().unwrap(),
            decrypted.to_str().unwrap(),
            "wrong_password"
        );

        assert!(result.is_err(), "Decryption with wrong password should fail");

        dir.close().unwrap();
    }

    #[test]
    fn test_aes_large_file() {
        let dir = tempdir().unwrap();
        let input = dir.path().join("large.txt");
        let encrypted = dir.path().join("encrypted.aes");
        let decrypted = dir.path().join("decrypted.txt");

        // Create a larger test file (1KB)
        let test_data = vec![b'A'; 1024];
        std::fs::write(&input, &test_data).unwrap();

        let password = "test_password";

        // Encrypt
        encrypt_file_aes(
            input.to_str().unwrap(),
            encrypted.to_str().unwrap(),
            password
        ).unwrap();

        // Decrypt
        decrypt_file_aes(
            encrypted.to_str().unwrap(),
            decrypted.to_str().unwrap(),
            password
        ).unwrap();

        // Verify
        let decrypted_data = std::fs::read(&decrypted).unwrap();
        assert_eq!(decrypted_data, test_data, "Large file should decrypt correctly");

        dir.close().unwrap();
    }
}
