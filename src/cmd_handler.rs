
use crate::{
    commands::{
            Decrypting, Encrypting, Hashing, StringSearch
    },
    cryptography::{
        file_decryption::decrypt_large_file,
        file_encryption::{digest_file_sha256, encrypt_large_file}, hash_algorithm::Algorithm,
    }

};
use anyhow::{Error, Result};

pub fn hash_handler(hashing: &Hashing) {
    match hashing.input_type.as_deref() {
        Some("string") => {
            let input = match hashing.input_string.as_ref() {
                Some(input) => input,
                None => {
                    return
                }
            };
            let result = hash_input(hashing.algorithm.as_ref().unwrap().as_str(),input);
            if result.is_err() {
                println!("Error occurred. {}", result.unwrap_err());
                return
            }
            println!("Result: {}", result.unwrap());
            return
        }
        Some("file") => {

        }
        _ => {
        }
    }
}

pub fn encrypt_handler(encrypting: &Encrypting) {
    match encrypting.input_type.as_deref() {
        Some("string") => {
            let input = match encrypting.input_string.as_ref() {
                Some(input) => input,
                None => {
                    return
                }
            };
            let result = hash_input(encrypting.algorithm.as_ref().unwrap().as_str(),input);
            if result.is_err() {
                println!("Error occurred. {}", result.unwrap_err());
                return
            }
            println!("Result: {}", result.unwrap());
            return
        }
        Some("file") => {
            let algorithm = encrypting.algorithm.as_ref().unwrap().as_str();
            let input_path = encrypting.file_path.as_deref().unwrap_or("inputfile");
            let output_path = encrypting.output_path.as_deref().unwrap_or("outputfile");
            let password = match encrypting.password.as_deref() {
                Some(password) => password,
                None => {
                    println!("Password missing.");
                    return
                }
            };
            match encrypt_file(algorithm, input_path, output_path, password){
                Ok(s) => println!("Encrypted file: {}", s),
                Err(e) => println!("Error occurred. {e}")
                // Err(e) => match e.downcast_ref() { // Can be used like this!
                //     // Some(MyErrors::LetsFixThisTomorrowError) => (),
                // }
            };
            println!("File is encrypted: {}", output_path);
            true
        }
        _ => {
            false
        }
    };
}

pub fn decrypt_handler(decrypting: &Decrypting) {
    if let Some(input_type) = decrypting.input_type.as_deref() {
        match input_type {
            "string" => {
                return;
            }
            "file" => {
                let output_path = decrypting.output_path.as_deref().unwrap_or("outputfile");
                let password = match decrypting.password.as_deref() {
                    Some(password) => password,
                    None => {
                        println!("Password missing.");
                        return
                    }
                };
                match decrypt_file(decrypting.algorithm.as_ref().unwrap().as_str(), decrypting.file_path.as_ref().unwrap().as_str(), output_path, password){
                    Ok(s) => println!("Decrypted file: {}", s),
                    Err(e) => println!("Error occurred. {e}")
                    // Err(e) => match e.downcast_ref() { // Can be used like this!
                    //     // Some(MyErrors::LetsFixThisTomorrowError) => (),
                    // }
                };
                return
            }
            _ => {
                println!("Unknown input type: {}", input_type);
                return;
            }
        }
    } else {
        // Handle the case where input_type is None
        println!("No input type provided.");
        return;
    }
}

fn hash_input(algorithm: &str, input_str: &str) -> Result<String, Error> {
    let result = match algorithm.to_lowercase().as_str() {
        "sha256" => Algorithm::Sha256.compute_hash(input_str),
        "sha512" => Algorithm::Sha512.compute_hash(input_str),
        "md5" => Algorithm::Md5.compute_hash(input_str),
        _ => return Err(anyhow::Error::msg("invalid algorithm input.")),
    };
    return result;
}
fn encrypt_file(algorithm: &str, file_path: &str, output_path: &str, password: &str) -> Result<String, Error> {
    match algorithm.to_lowercase().as_str() {
        "sha256" => {
            let result = digest_file_sha256(file_path)?;
            Ok(format!("File encrypted using sha256 algorithm. {}", result))
        },
        "default" => {
            encrypt_large_file(file_path, output_path, password)?;
            Ok(format!("File encrypted successfully to {}", output_path))
        },
        _ => Err(Error::msg("Invalid algorithm input.")),
    }
}
fn decrypt_file(algorithm: &str, file_path: &str, output_path: &str, password: &str) -> Result<String, Error> {
    match algorithm.to_lowercase().as_str() {

        "default" => {
            decrypt_large_file(file_path, output_path, password)?;
            Ok(format!("File encrypted successfully to {}", output_path))
        },
        _ => Err(Error::msg("Invalid algorithm input.")),
    }
}
pub fn string_handler(stringhandler: &StringSearch) {
    match stringhandler.input_string {
        Some(ref _str) => {

        }None => {}
    }

    match stringhandler.search {
        Some(ref _search) => {

        }None => {}
    }
}