use std::fs::File;
use std::io::{Read, Write};
use orion::hazardous::{
    aead::xchacha20poly1305::{open, Nonce, SecretKey},
    mac::poly1305::POLY1305_OUTSIZE,
    stream::xchacha20::XCHACHA_NONCESIZE,
};

use orion::hazardous::stream::chacha20::CHACHA_KEYSIZE;

use super::common::create_key;
const CHUNK_SIZE: usize = 128;

/// Detect the encryption version from the file
/// Returns 0x00 for legacy (no version byte), 0x01 for XChaCha20-Poly1305
fn detect_encryption_version(data: &[u8]) -> u8 {
    if data.is_empty() {
        return 0x00;
    }

    // Check if first byte looks like a version byte (0x01-0x03)
    // Legacy files start with a 24-byte nonce (random bytes)
    if data[0] >= 0x01 && data[0] <= 0x03 {
        data[0]
    } else {
        0x00  // Legacy format
    }
}




pub fn decrypt_large_file(
    file_path: &str,
    output_path: &str,
    password: &str
) -> Result<(), orion::errors::UnknownCryptoError> {
    let mut input_file = File::open(file_path).expect("Failed to open input file");
    let mut output_file = File::create(output_path).expect("Failed to create output file");

    let mut src: Vec<u8> = Vec::new();
    input_file.read_to_end(&mut src).expect("Failed to read input file");

    // Detect encryption version
    let version = detect_encryption_version(&src);

    let nonce_offset = match version {
        0x01 => {
            // New format: skip version byte (1 byte)
            1
        }
        0x00 => {
            // Legacy format: no version byte
            println!("Warning: Decrypting legacy format file (pre-v0.1). Consider re-encrypting with current version.");
            0
        }
        _ => {
            panic!("Unsupported encryption version: 0x{:02X}", version);
        }
    };

    let nonce = src[nonce_offset..nonce_offset + XCHACHA_NONCESIZE].to_vec();
    src = src[nonce_offset + XCHACHA_NONCESIZE..].to_vec();

    let key = create_key(password, nonce.clone());
    let nonce = Nonce::from_slice(nonce.as_slice()).unwrap();

    for (n_chunk, src_chunk) in src.chunks(CHUNK_SIZE + CHACHA_KEYSIZE + POLY1305_OUTSIZE).enumerate() {
        decrypt_core(&mut output_file, src_chunk.to_vec(), &key, nonce);
    }

    Ok(())
}


fn simple_split_encrypted(cipher_text: &[u8]) -> (Vec<u8>, Vec<u8>) {
    return (
        cipher_text[..CHACHA_KEYSIZE].to_vec(),
        cipher_text[CHACHA_KEYSIZE..].to_vec(),
        )
}

fn decrypt_core(
    dist: &mut File,
    contents: Vec<u8>,
    key: &SecretKey,
    nonce: Nonce
) {
    let split = simple_split_encrypted(contents.as_slice());
    let mut output = vec![0u8; split.1.len() - POLY1305_OUTSIZE];

    open(&key, &nonce, split.1.as_slice(), Some(split.0.as_slice()), &mut output).unwrap();
    dist.write(&output.as_slice()).unwrap();
}