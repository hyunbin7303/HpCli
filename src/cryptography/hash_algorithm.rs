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
