use std::{fs::File, io::Read};

// use crypto::aes;
use sha2::{Digest, Sha256};



pub fn encrypt_sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let hash = format!("{:X}", hasher.finalize());
    return hash;
}


// Hash binary file?
//https://stackoverflow.com/questions/69787906/how-to-hash-a-binary-file-in-rust


// pub fn compute_sha256_hash_file(file: &str) -> Result<(), String> {
//     let mut file = File::open(file)?;
//     let mut hasher = Sha256::new();
//     let mut buffer = [0; 4096];
//     loop {
//         let bytes_read = file.read(&mut buffer)?;
//         if bytes_read ==0 {
//             break;
//         }
//         hasher.update(&buffer[..bytes_read]);
//     }
//     let hash = format!("{:X}", hasher.finalize());

//     Ok(hash)
// }
