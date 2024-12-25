use std::fs::File;
use std::error::Error;
use std::io::{BufReader, Read};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use rand::RngCore;
use rand_core::OsRng;
use anyhow::Result;
use std::io::Write;
use orion::{hazardous::stream::chacha20::CHACHA_KEYSIZE, kdf::{derive_key, Password, Salt}};

use orion::hazardous::{
    aead::xchacha20poly1305::{seal, Nonce, SecretKey},
    mac::poly1305::POLY1305_OUTSIZE,
};
use sha2::{Digest, Sha256};

const CHUNK_SIZE: usize = 128;

#[derive(Debug)]
pub struct CryptoError;
impl Error for CryptoError {}
impl Display for CryptoError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!( f, "Crypto failure" )
    }
}
pub fn digest_file_sha256(file_path: &str) -> Result<String> {
    let input = File::open(file_path)?;
    let mut reader = BufReader::new(input);

    let digest = {
        let mut hasher = Sha256::new();
        let mut buffer = [0; 1024];
        loop {
            let count = reader.read(&mut buffer)?;
            if count == 0 { break }
            hasher.update(&buffer[..count]);
        }
        hasher.finalize()
    };
    let result = format!("{:X}", digest);

    Ok(result)
}

fn get_random(dest: &mut [u8]) {
    RngCore::fill_bytes(&mut OsRng, dest);
}
fn nonce() -> Vec<u8> {
    let mut randoms: [u8; 24] = [0; 24];
    get_random(&mut randoms);
    return randoms.to_vec();
}

fn auth_tag() -> Vec<u8> {
    let mut randoms: [u8; 32] = [0; 32];
    get_random(&mut randoms);
    return randoms.to_vec();
}

fn simple_split_encrypted(cipher_text: &[u8]) -> (Vec<u8>, Vec<u8>) {
    return (
        cipher_text[..CHACHA_KEYSIZE].to_vec(),
        cipher_text[CHACHA_KEYSIZE..].to_vec(),
        )
}
fn create_key(password: &str, nonce: Vec<u8>) -> SecretKey {
    let password = Password::from_slice(password.as_bytes()).unwrap();
    let salt = Salt::from_slice(nonce.as_slice()).unwrap();
    let kdf_key = derive_key(&password, &salt, 15, 1024, CHACHA_KEYSIZE as u32).unwrap();
    let key = SecretKey::from_slice(kdf_key.unprotected_as_bytes()).unwrap();
    return key;
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

pub fn encrypt_large_file(file_path: &str, output_path: &str, password: &str)
    -> Result<(), orion::errors::UnknownCryptoError>
{
    let mut source_file = File::open(file_path).expect("Failed to open input file");
    let mut dist = File::create(output_path).expect("Failed to create output file");

    let mut src = Vec::new();
    source_file.read_to_end(&mut src).expect("Failed to read input file");

    let nonce = nonce();

    dist.write(nonce.as_slice()).unwrap();
    let key = create_key(password, nonce.clone());
    let nonce = Nonce::from_slice(nonce.as_slice()).unwrap();

    for (n_chunk, src_chunk) in src.chunks(CHUNK_SIZE).enumerate() {
        encrypt_core(&mut dist, src_chunk.to_vec(), &key, nonce)
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
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


    // TODO Validate this test method.
    #[test]
    fn test_encrypt_large_file_input_not_found() {
        // Use a non-existent input file path
        let input_file_path = "non_existent_input.txt";
        let output_file_path = "output.enc";
        let password = "super_secret_password";

        // Call the encrypt_large_file function and expect an error
        let result = encrypt_large_file(input_file_path, output_file_path, password);

        // Assert that an error is returned
        assert!(result.is_err());
    }

}
//https://dev.to/vapourisation/file-encryption-in-rust-3kid