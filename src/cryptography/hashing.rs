use sha2::{Digest, Sha256, Sha512};
use anyhow::Result;

pub fn encrypt_sha256(input: &str) -> Result<String> {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let hash = format!("{:X}", hasher.finalize());
    return Ok(hash);
}
pub fn encrypt_sha512(input: &str) -> Result<String> {
    let mut hasher = Sha512::new();
    hasher.update(input);
    let hash = format!("{:X}", hasher.finalize());
    return Ok(hash);
}
pub fn encrypt_md5(input: &str) -> Result<String> {
    let digest = md5::compute(input);
    let result = format!("{:X}", digest);
    return Ok(result);
}
