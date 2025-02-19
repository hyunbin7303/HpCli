use std::{fs::File, io::{BufReader, Read}};

use sha2::{Digest, Sha256, Sha512};
use anyhow::Result;


pub enum Algorithm {
    Sha256,
    Sha512,
    Md5,

}
impl Algorithm {
    pub fn compute_hash(&self, input: &str) -> Result<String> {
        match self {
            Algorithm::Sha256 => {
                let mut hasher = Sha256::new();
                hasher.update(input);
                let hash = format!("{:X}", hasher.finalize());
                Ok(hash)
            }
            Algorithm::Sha512 => {
                let mut hasher = Sha512::new();
                hasher.update(input);
                let hash = format!("{:X}", hasher.finalize());
                Ok(hash)
            }
            Algorithm::Md5 => {
                let digest = md5::compute(input);
                let hash = format!("{:X}", digest);
                Ok(hash)
            }
        }
    }
}


pub fn digest_file_md5(file_path: &str) -> Result<String> {
    let input = File::open(file_path)?;
    let mut reader = BufReader::new(input);

    let mut context = md5::Context::new(); // Create a new MD5 context
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.consume(&buffer[..count]);
    }
    let digest = context.compute();// Finalize the hash and format it as a hexadecimal string
    Ok(format!("{:x}", digest))
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