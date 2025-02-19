use std::fs::File;
use std::error::Error;
use std::io::{BufReader, Read};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use anyhow::Result;
use std::io::Write;
use orion::hazardous::stream::chacha20::CHACHA_KEYSIZE;
use orion::hazardous::{
    aead::chacha20poly1305::{seal, Nonce, SecretKey},
    mac::poly1305::POLY1305_OUTSIZE,
};
use sha2::{Digest, Sha256};
use md5;
use super::common::{auth_tag, create_key, nonce};

const CHUNK_SIZE: usize = 128;

#[derive(Debug)]
pub struct CryptoError;
impl Error for CryptoError {}
impl Display for CryptoError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!( f, "Crypto failure" )
    }
}

fn encrypt_core(
    dist: &mut File,
    contents: Vec<u8>,
    key: &SecretKey,
    nonce: Nonce,
) {
    let ad = auth_tag();
    let output_len = match contents.len().checked_add(POLY1305_OUTSIZE + ad.len()) {
        Some(min_output_len) => min_output_len,
        None => panic!("Plaintext is too long"),
    };

    let mut output = vec![0u8; output_len];
    output[..CHACHA_KEYSIZE].copy_from_slice(ad.as_ref());
    seal(&key, &nonce, contents.as_slice(), Some(ad.clone().as_slice()), &mut output[CHACHA_KEYSIZE..]).unwrap();
    dist.write(&output.as_slice()).unwrap();
}

pub fn encrypt_large_file(file_path: &str,output_path: &str,password: &str)
    -> Result<(), orion::errors::UnknownCryptoError> {
    let mut source_file = File::open(file_path).expect("Failed to open input file");
    let mut dist = File::create(output_path).expect("Failed to create output file");

    let mut src = Vec::new();
    source_file.read_to_end(&mut src).expect("Failed to read input file");

    let nonce = nonce();

    dist.write(nonce.as_slice()).unwrap();
    let key = create_key(password, nonce.clone());
    let nonce = Nonce::from_slice(nonce.as_slice()).unwrap();

    for (_n_chunk, src_chunk) in src.chunks(CHUNK_SIZE).enumerate() {
        encrypt_core(&mut dist, src_chunk.to_vec(), &key, nonce)
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::panic;
    use tempfile::tempdir;

    #[test]
    fn test_encrypt_large_file_success() {
        // Create a temporary directory
        let dir = tempdir().unwrap();
        let input_file_path = dir.path().join("input.txt");
        let output_file_path = dir.path().join("kevinoutput.enc");

        // Write some data to the input file
        let mut input_file = File::create(&input_file_path).unwrap();
        writeln!(input_file, "This is a test file.").unwrap();

        let password = "strongpassword";
        let result = encrypt_large_file(
            input_file_path.to_str().unwrap(),
            output_file_path.to_str().unwrap(),
            password,
        );

        assert!(result.is_ok());
        let metadata = fs::metadata(output_file_path).unwrap();
        assert!(metadata.is_file());
        assert!(metadata.len() > 0);
        dir.close().unwrap();
    }

    #[test]
    fn test_encrypt_large_file_invalid_input_path() {
        let invalid_input_path = "nonexistent_input_file.txt";
        let output_path = "output_file.txt";
        let password = "securepassword";

        // Act: Call the function and capture the result
        let result = panic::catch_unwind(|| {
            let _ = encrypt_large_file(invalid_input_path, output_path, password);
            panic!("This is a test panic!");
        });

        assert!(result.is_err(), "Expected an error for invalid input file path");

        // Cleanup: Ensure no output file is created
        assert!(!fs::metadata(output_path).is_ok(), "Output file should not be created");
    }

}