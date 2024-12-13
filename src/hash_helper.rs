use std::{fs::File, io::{BufReader, Read}, path::PathBuf};
use sha2::{Digest, Sha256};
use anyhow::Result;

pub fn encrypt_sha256(input: &str) -> Result<String> {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let hash = format!("{:X}", hasher.finalize());
    return Ok(hash);
}

pub fn encrypt_md5(input: &str) -> Result<String> {
    let digest = md5::compute(input);
    let result = format!("{:X}", digest);
    return Ok(result);
}

fn digest_file_sha256(path: &PathBuf) -> Result<String> {
    let input = File::open(path)?;
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