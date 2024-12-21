use crate::{commands::{Encrypting, StringSearch}, cryptography::hash_helper::{encrypt_md5, encrypt_sha256, encrypt_sha512}};




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
                println!("Error comes out. {}", result.unwrap_err());
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
            println!("input file validation: {}", input);
            true
        }
        _ => {
            false
        }
    };
}

fn encrypt_string(algorithm: &str, input_str: &str) -> Result<String, anyhow::Error> {
    let result = match algorithm.to_lowercase().as_str() {
        "sha256" => encrypt_sha256(input_str),
        "sha512" => encrypt_sha512(input_str),
        "md5" => encrypt_md5(input_str),
        _ => return Err(anyhow::Error::msg("invalid algorithm input.")),
    };
    return result;
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