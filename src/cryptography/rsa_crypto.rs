use anyhow::{Error, Result};
use rsa::{RsaPrivateKey, RsaPublicKey, Oaep, PublicKey};
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey, DecodePrivateKey, DecodePublicKey, LineEnding};
use aes_gcm::{Aes256Gcm, Key, Nonce, aead::{Aead, KeyInit, OsRng, rand_core::RngCore}};
use std::fs;
use std::io::{Read, Write};

/// Generate an RSA keypair and save to files
/// Files created: {output_path}_private.pem and {output_path}_public.pem
pub fn generate_rsa_keypair(bits: usize, output_path: &str) -> Result<()> {
    let mut rng = rand::thread_rng();

    // Generate RSA keypair
    let private_key = RsaPrivateKey::new(&mut rng, bits)
        .map_err(|e| Error::msg(format!("Failed to generate RSA private key: {}", e)))?;

    let public_key = RsaPublicKey::from(&private_key);

    // Encode to PEM format
    let private_pem = private_key.to_pkcs8_pem(LineEnding::LF)
        .map_err(|e| Error::msg(format!("Failed to encode private key to PEM: {}", e)))?;

    let public_pem = public_key.to_public_key_pem(LineEnding::LF)
        .map_err(|e| Error::msg(format!("Failed to encode public key to PEM: {}", e)))?;

    // Save keys to files
    let private_path = format!("{}_private.pem", output_path);
    let public_path = format!("{}_public.pem", output_path);

    fs::write(&private_path, private_pem.as_bytes())
        .map_err(|e| Error::msg(format!("Failed to write private key: {}", e)))?;

    fs::write(&public_path, public_pem)
        .map_err(|e| Error::msg(format!("Failed to write public key: {}", e)))?;

    println!("RSA-{} keypair generated successfully:", bits);
    println!("  Private key: {}", private_path);
    println!("  Public key: {}", public_path);
    println!("\nSecurity reminders:");
    println!("  - Keep your private key secure (chmod 600 recommended)");
    println!("  - Never share your private key");
    println!("  - The public key can be freely distributed");

    Ok(())
}

/// Encrypt a file using hybrid RSA+AES encryption
/// For files > 190 bytes: encrypts file with AES-256-GCM, then encrypts AES key with RSA
/// File format: [version 0x03][2-byte key length][RSA-encrypted AES key][12-byte nonce][AES-GCM ciphertext]
pub fn encrypt_file_rsa(
    input_path: &str,
    output_path: &str,
    public_key_path: &str
) -> Result<()> {
    // Load public key
    let public_pem = fs::read_to_string(public_key_path)
        .map_err(|e| Error::msg(format!("Failed to read public key: {}", e)))?;

    let public_key = RsaPublicKey::from_public_key_pem(&public_pem)
        .map_err(|e| Error::msg(format!("Failed to parse public key: {}", e)))?;

    // Read plaintext file
    let plaintext = fs::read(input_path)
        .map_err(|e| Error::msg(format!("Failed to read input file: {}", e)))?;

    // Use hybrid encryption (RSA + AES) for all files
    // This is necessary because RSA can only encrypt small amounts of data

    // 1. Generate random AES-256 key
    let mut aes_key_bytes = [0u8; 32];
    OsRng.fill_bytes(&mut aes_key_bytes);

    // 2. Generate random nonce for AES-GCM
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);

    // 3. Encrypt file with AES-256-GCM
    let aes_key = Key::<Aes256Gcm>::from_slice(&aes_key_bytes);
    let cipher = Aes256Gcm::new(aes_key);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, plaintext.as_ref())
        .map_err(|e| Error::msg(format!("AES encryption failed: {}", e)))?;

    // 4. Encrypt AES key with RSA-OAEP
    let padding = Oaep::new::<sha2::Sha256>();
    let mut rng = rand::thread_rng();

    let encrypted_key = public_key.encrypt(&mut rng, padding, &aes_key_bytes)
        .map_err(|e| Error::msg(format!("RSA encryption failed: {}", e)))?;

    // 5. Write hybrid format to file
    let mut output = fs::File::create(output_path)
        .map_err(|e| Error::msg(format!("Failed to create output file: {}", e)))?;

    // Write header: version byte
    output.write_all(&[0x03])?;

    // Write length of encrypted AES key (2 bytes, big-endian)
    let key_len = encrypted_key.len() as u16;
    output.write_all(&key_len.to_be_bytes())?;

    // Write RSA-encrypted AES key
    output.write_all(&encrypted_key)?;

    // Write nonce
    output.write_all(&nonce_bytes)?;

    // Write AES-GCM ciphertext (includes authentication tag)
    output.write_all(&ciphertext)?;

    Ok(())
}

/// Decrypt a file using hybrid RSA+AES decryption
pub fn decrypt_file_rsa(
    input_path: &str,
    output_path: &str,
    private_key_path: &str
) -> Result<()> {
    // Load private key
    let private_pem = fs::read_to_string(private_key_path)
        .map_err(|e| Error::msg(format!("Failed to read private key: {}", e)))?;

    let private_key = RsaPrivateKey::from_pkcs8_pem(&private_pem)
        .map_err(|e| Error::msg(format!("Failed to parse private key: {}", e)))?;

    // Read encrypted file
    let mut file = fs::File::open(input_path)
        .map_err(|e| Error::msg(format!("Failed to open input file: {}", e)))?;

    let mut encrypted_data = Vec::new();
    file.read_to_end(&mut encrypted_data)?;

    // Parse header
    if encrypted_data.is_empty() || encrypted_data[0] != 0x03 {
        return Err(Error::msg("Invalid file format or version (expected RSA hybrid encryption v0x03)"));
    }

    if encrypted_data.len() < 4 {
        return Err(Error::msg("File too small to be valid"));
    }

    // Read encrypted key length
    let key_len = u16::from_be_bytes([encrypted_data[1], encrypted_data[2]]) as usize;

    if encrypted_data.len() < 3 + key_len + 12 {
        return Err(Error::msg("File corrupted or truncated"));
    }

    // Extract components
    let encrypted_key = &encrypted_data[3..3 + key_len];
    let nonce_bytes = &encrypted_data[3 + key_len..3 + key_len + 12];
    let ciphertext = &encrypted_data[3 + key_len + 12..];

    // Decrypt AES key with RSA-OAEP
    let padding = Oaep::new::<sha2::Sha256>();
    let aes_key_bytes = private_key.decrypt(padding, encrypted_key)
        .map_err(|e| Error::msg(format!("RSA decryption failed (wrong key or corrupted file): {}", e)))?;

    if aes_key_bytes.len() != 32 {
        return Err(Error::msg("Decrypted AES key has invalid length"));
    }

    // Decrypt file with AES-256-GCM
    let aes_key = Key::<Aes256Gcm>::from_slice(&aes_key_bytes);
    let cipher = Aes256Gcm::new(aes_key);
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher.decrypt(nonce, ciphertext)
        .map_err(|e| Error::msg(format!("AES decryption failed (corrupted file): {}", e)))?;

    // Write decrypted data
    fs::write(output_path, plaintext)
        .map_err(|e| Error::msg(format!("Failed to write output file: {}", e)))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_rsa_keypair_generation() {
        let dir = tempdir().unwrap();
        let key_path = dir.path().join("test_key");

        let result = generate_rsa_keypair(2048, key_path.to_str().unwrap());
        assert!(result.is_ok(), "Keypair generation should succeed");

        let private_path = format!("{}_private.pem", key_path.to_str().unwrap());
        let public_path = format!("{}_public.pem", key_path.to_str().unwrap());

        assert!(fs::metadata(&private_path).is_ok(), "Private key file should exist");
        assert!(fs::metadata(&public_path).is_ok(), "Public key file should exist");

        // Verify keys can be loaded
        let private_pem = fs::read_to_string(&private_path).unwrap();
        let public_pem = fs::read_to_string(&public_path).unwrap();

        assert!(RsaPrivateKey::from_pkcs8_pem(&private_pem).is_ok(), "Private key should be valid");
        assert!(RsaPublicKey::from_public_key_pem(&public_pem).is_ok(), "Public key should be valid");

        dir.close().unwrap();
    }

    #[test]
    fn test_rsa_hybrid_encryption_roundtrip() {
        let dir = tempdir().unwrap();
        let key_path = dir.path().join("rsa_key");
        let input = dir.path().join("plaintext.txt");
        let encrypted = dir.path().join("encrypted.rsa");
        let decrypted = dir.path().join("decrypted.txt");

        // Test data
        let test_data = b"This is a test file for RSA hybrid encryption with AES-256-GCM!";
        fs::write(&input, test_data).unwrap();

        // Generate keypair
        generate_rsa_keypair(2048, key_path.to_str().unwrap()).unwrap();

        let public_key = format!("{}_public.pem", key_path.to_str().unwrap());
        let private_key = format!("{}_private.pem", key_path.to_str().unwrap());

        // Encrypt
        let encrypt_result = encrypt_file_rsa(
            input.to_str().unwrap(),
            encrypted.to_str().unwrap(),
            &public_key
        );
        assert!(encrypt_result.is_ok(), "Encryption should succeed");

        // Verify encrypted file has correct version byte
        let encrypted_data = fs::read(&encrypted).unwrap();
        assert_eq!(encrypted_data[0], 0x03, "Version byte should be 0x03");

        // Decrypt
        let decrypt_result = decrypt_file_rsa(
            encrypted.to_str().unwrap(),
            decrypted.to_str().unwrap(),
            &private_key
        );
        assert!(decrypt_result.is_ok(), "Decryption should succeed");

        // Verify content matches
        let decrypted_data = fs::read(&decrypted).unwrap();
        assert_eq!(decrypted_data, test_data, "Decrypted content should match original");

        dir.close().unwrap();
    }

    #[test]
    fn test_rsa_large_file() {
        let dir = tempdir().unwrap();
        let key_path = dir.path().join("rsa_key");
        let input = dir.path().join("large.txt");
        let encrypted = dir.path().join("encrypted.rsa");
        let decrypted = dir.path().join("decrypted.txt");

        // Create a larger file (10KB)
        let test_data = vec![b'X'; 10240];
        fs::write(&input, &test_data).unwrap();

        // Generate keypair
        generate_rsa_keypair(2048, key_path.to_str().unwrap()).unwrap();

        let public_key = format!("{}_public.pem", key_path.to_str().unwrap());
        let private_key = format!("{}_private.pem", key_path.to_str().unwrap());

        // Encrypt and decrypt
        encrypt_file_rsa(input.to_str().unwrap(), encrypted.to_str().unwrap(), &public_key).unwrap();
        decrypt_file_rsa(encrypted.to_str().unwrap(), decrypted.to_str().unwrap(), &private_key).unwrap();

        // Verify
        let decrypted_data = fs::read(&decrypted).unwrap();
        assert_eq!(decrypted_data, test_data, "Large file should decrypt correctly");

        dir.close().unwrap();
    }
}
