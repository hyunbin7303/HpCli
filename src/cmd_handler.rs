
use crate::{commands::{Encrypting, StringSearch}, cryptography::hash_helper::{encrypt_large_file, encrypt_md5, encrypt_sha256, encrypt_sha512}};
use anyhow::{Error, Result};



pub fn crypto_handler(encrypting: &Encrypting) {
    match encrypting.input_type.as_deref() { // TODO: Is this right approach?
        Some("string") => {
            let input = match encrypting.input_string.as_ref() {
                Some(input) => input,
                None => {
                    return
                }
            };
            let result = encrypt_string(encrypting.algorithm.as_ref().unwrap().as_str(),input);
            if result.is_err() {
                println!("Error occurred. {}", result.unwrap_err());
                return
            }
            println!("Result: {}", result.unwrap());
            return
        }
        Some("file") => {
            // let input = match encrypting.file_path.
            let input = match encrypting.input_string.as_ref() {
                Some(input) => input,
                None => {
                    return
                }
            };
            let output_path = encrypting.output_path.as_deref().unwrap_or("outputfile");
            let password = match encrypting.password.as_deref() {
                Some(password) => password,
                None => {
                    println!("Password missing.");
                    return
                }
            };
            let result = encrypt_file(encrypting.algorithm.as_ref().unwrap().as_str(), input, output_path, password);
            if result.is_err() {
                println!("Error occurred. {}", result.unwrap_err());
                return
            }
            println!("File is encrypted: {}", output_path);
            true
        }
        _ => {
            false
        }
    };
}

fn encrypt_string(algorithm: &str, input_str: &str) -> Result<String, Error> {
    let result = match algorithm.to_lowercase().as_str() {
        "sha256" => encrypt_sha256(input_str),
        "sha512" => encrypt_sha512(input_str),
        "md5" => encrypt_md5(input_str),
        _ => return Err(anyhow::Error::msg("invalid algorithm input.")),
    };
    return result;
}
fn encrypt_file(algorithm: &str, file_path: &str, output_path: &str, password: &str) -> Result<String, Error> {
    match algorithm.to_lowercase().as_str() {
        "default" => {
            encrypt_large_file(file_path, output_path, password)?;
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