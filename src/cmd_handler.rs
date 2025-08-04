
use crate::{
    commands::{
            Decrypting, Encrypting, Hashing, StringSearch, Jwt
    },
    cryptography::{
        file_decryption::decrypt_large_file,
        file_encryption::{ encrypt_large_file}, hash_algorithm::Algorithm,
        jwt_keygen::{generate_jwt_secret, save_key_to_file, JwtAlgorithm, OutputFormat},
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
        // "aes" => Algorithm:: // Check for the algorithm
        _ => return Err(anyhow::Error::msg("invalid algorithm input. Please use among [sha256, sha512, md5, aes]")),
    };
    return result;
}
fn encrypt_file(algorithm: &str, file_path: &str, output_path: &str, password: &str) -> Result<String, Error> {
    match algorithm.to_lowercase().as_str() {
        // "sha256" => {
        //     let result = digest_file_sha256(file_path)?;
        //     Ok(format!("File encrypted using sha256 algorithm. {}", result))
        // },
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

pub fn jwt_handler(jwt: &Jwt) {
    let algorithm_str = match jwt.algorithm.as_ref() {
        Some(alg) => alg,
        None => {
            println!("Algorithm is required. Please specify one of: HS256, HS384, HS512, RS256, RS384, RS512");
            return;
        }
    };

    let algorithm = match JwtAlgorithm::from_str(algorithm_str) {
        Ok(alg) => alg,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    let format = match OutputFormat::from_str(&jwt.format) {
        Ok(fmt) => fmt,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    println!("Generating JWT secret key for algorithm: {}", algorithm_str);
    
    match generate_jwt_secret(algorithm, format, jwt.key_size) {
        Ok(key_pair) => {
            if let Some(ref public_key) = key_pair.public_key {
                println!("=== RSA Key Pair Generated ===");
                println!("\n--- Private Key ---");
                println!("{}", key_pair.private_key);
                println!("\n--- Public Key ---");
                println!("{}", public_key);

                if let Some(ref output_path) = jwt.output_path {
                    let private_path = format!("{}_private.pem", output_path);
                    let public_path = format!("{}_public.pem", output_path);
                    
                    match save_key_to_file(&key_pair.private_key, &private_path) {
                        Ok(_) => println!("\nPrivate key saved to: {}", private_path),
                        Err(e) => println!("Error saving private key: {}", e),
                    }
                    
                    match save_key_to_file(public_key, &public_path) {
                        Ok(_) => println!("Public key saved to: {}", public_path),
                        Err(e) => println!("Error saving public key: {}", e),
                    }
                }
            } else {
                println!("=== HMAC Secret Key Generated ===");
                println!("Secret Key: {}", key_pair.private_key);
                println!("Key Length: {} bytes", 
                    match algorithm_str.to_uppercase().as_str() {
                        "HS256" => "32",
                        "HS384" => "48", 
                        "HS512" => "64",
                        _ => "unknown"
                    }
                );

                if let Some(ref output_path) = jwt.output_path {
                    match save_key_to_file(&key_pair.private_key, output_path) {
                        Ok(_) => println!("\nSecret key saved to: {}", output_path),
                        Err(e) => println!("Error saving key: {}", e),
                    }
                }
            }

            println!("\n=== Usage Instructions ===");
            if key_pair.public_key.is_some() {
                println!("For RSA algorithms (RS256/RS384/RS512):");
                println!("- Use the PRIVATE key to SIGN JWT tokens");
                println!("- Use the PUBLIC key to VERIFY JWT tokens");
                println!("- Keep the private key secure and never expose it");
                println!("- The public key can be shared for token verification");
            } else {
                println!("For HMAC algorithms (HS256/HS384/HS512):");
                println!("- Use this secret key for both signing AND verifying JWT tokens");
                println!("- Keep this key secure and never expose it");
                println!("- Both token creation and verification require the same secret");
            }
        }
        Err(e) => {
            println!("Error generating JWT secret key: {}", e);
        }
    }
}