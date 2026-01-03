
use crate::{
    commands::{
            Decrypting, Encrypting, Hashing, StringSearch, Jwt, Keygen
    },
    cryptography::{
        file_decryption::decrypt_large_file,
        file_encryption::{ encrypt_large_file},
        hash_algorithm::Algorithm,
        hmac_algorithm::HmacAlgorithm,
        aes_encryption::{encrypt_file_aes, decrypt_file_aes},
        rsa_crypto::{generate_rsa_keypair, encrypt_file_rsa, decrypt_file_rsa},
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

            // Check if HMAC mode is requested
            let result = if let Some(hmac_key) = &hashing.hmac_key {
                compute_hmac(
                    hashing.algorithm.as_ref().unwrap().as_str(),
                    hmac_key.as_bytes(),
                    input.as_bytes()
                )
            } else {
                hash_input(hashing.algorithm.as_ref().unwrap().as_str(), input)
            };

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

            // Check if this is RSA encryption (requires key instead of password)
            if algorithm.to_lowercase().starts_with("rsa") {
                let key_path = match encrypting.key_path.as_deref() {
                    Some(key) => key,
                    None => {
                        println!("Public key path required for RSA encryption (use --key)");
                        return;
                    }
                };
                match encrypt_file_rsa(input_path, output_path, key_path) {
                    Ok(_) => println!("Encrypted file with RSA: {}", output_path),
                    Err(e) => println!("Error occurred: {}", e),
                };
            } else {
                // Symmetric encryption (requires password)
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
                };
            }
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
                let algorithm = decrypting.algorithm.as_ref().unwrap().as_str();
                let input_path = decrypting.file_path.as_ref().unwrap().as_str();
                let output_path = decrypting.output_path.as_deref().unwrap_or("outputfile");

                // Check if this is RSA decryption (requires key instead of password)
                if algorithm.to_lowercase().starts_with("rsa") {
                    let key_path = match decrypting.key_path.as_deref() {
                        Some(key) => key,
                        None => {
                            println!("Private key path required for RSA decryption (use --key)");
                            return;
                        }
                    };
                    match decrypt_file_rsa(input_path, output_path, key_path) {
                        Ok(_) => println!("Decrypted file with RSA: {}", output_path),
                        Err(e) => println!("Error occurred: {}", e),
                    };
                } else {
                    // Symmetric decryption (requires password)
                    let password = match decrypting.password.as_deref() {
                        Some(password) => password,
                        None => {
                            println!("Password missing.");
                            return
                        }
                    };
                    match decrypt_file(algorithm, input_path, output_path, password){
                        Ok(s) => println!("Decrypted file: {}", s),
                        Err(e) => println!("Error occurred. {e}")
                    };
                }
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
        "sha3-256" => Algorithm::Sha3_256.compute_hash(input_str),
        "sha3-512" => Algorithm::Sha3_512.compute_hash(input_str),
        "blake2b" => Algorithm::Blake2b.compute_hash(input_str),
        "blake2s" => Algorithm::Blake2s.compute_hash(input_str),
        _ => return Err(anyhow::Error::msg("invalid algorithm input. Please use among [sha256, sha512, md5, sha3-256, sha3-512, blake2b, blake2s]")),
    };
    return result;
}

fn compute_hmac(algorithm: &str, key: &[u8], message: &[u8]) -> Result<String, Error> {
    match algorithm.to_lowercase().as_str() {
        "sha256" => HmacAlgorithm::HmacSha256.compute_hmac(key, message),
        "sha512" => HmacAlgorithm::HmacSha512.compute_hmac(key, message),
        "sha3-256" => HmacAlgorithm::HmacSha3_256.compute_hmac(key, message),
        "sha3-512" => HmacAlgorithm::HmacSha3_512.compute_hmac(key, message),
        _ => Err(anyhow::Error::msg("invalid HMAC algorithm. Please use among [sha256, sha512, sha3-256, sha3-512]. Note: Blake2 has built-in keyed hashing support.")),
    }
}
fn encrypt_file(algorithm: &str, file_path: &str, output_path: &str, password: &str) -> Result<String, Error> {
    match algorithm.to_lowercase().as_str() {
        "default" | "xchacha20" => {
            encrypt_large_file(file_path, output_path, password)?;
            Ok(format!("File encrypted successfully using XChaCha20-Poly1305 to {}", output_path))
        },
        "aes256gcm" | "aes-256-gcm" => {
            encrypt_file_aes(file_path, output_path, password)?;
            Ok(format!("File encrypted successfully using AES-256-GCM to {}", output_path))
        },
        _ => Err(Error::msg("Invalid algorithm. Use: default, xchacha20, aes256gcm")),
    }
}
fn decrypt_file(algorithm: &str, file_path: &str, output_path: &str, password: &str) -> Result<String, Error> {
    match algorithm.to_lowercase().as_str() {
        "default" | "xchacha20" => {
            decrypt_large_file(file_path, output_path, password)?;
            Ok(format!("File decrypted successfully using XChaCha20-Poly1305 to {}", output_path))
        },
        "aes256gcm" | "aes-256-gcm" => {
            decrypt_file_aes(file_path, output_path, password)?;
            Ok(format!("File decrypted successfully using AES-256-GCM to {}", output_path))
        },
        _ => Err(Error::msg("Invalid algorithm. Use: default, xchacha20, aes256gcm")),
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

pub fn keygen_handler(keygen: &Keygen) {
    let algorithm = match keygen.algorithm.as_ref() {
        Some(alg) => alg,
        None => {
            println!("Algorithm is required. Please specify: rsa2048, rsa3072, rsa4096, or ecdsa-p256");
            return;
        }
    };

    let output_path = match keygen.output_path.as_ref() {
        Some(path) => path,
        None => {
            println!("Output path is required");
            return;
        }
    };

    match algorithm.to_lowercase().as_str() {
        "rsa2048" => {
            match generate_rsa_keypair(2048, output_path) {
                Ok(_) => {},
                Err(e) => println!("Error generating RSA-2048 keypair: {}", e),
            }
        },
        "rsa3072" => {
            match generate_rsa_keypair(3072, output_path) {
                Ok(_) => {},
                Err(e) => println!("Error generating RSA-3072 keypair: {}", e),
            }
        },
        "rsa4096" => {
            match generate_rsa_keypair(4096, output_path) {
                Ok(_) => {},
                Err(e) => println!("Error generating RSA-4096 keypair: {}", e),
            }
        },
        "ecdsa-p256" => {
            println!("ECDSA key generation will be available in Phase 5");
        },
        _ => {
            println!("Unsupported algorithm: {}. Use: rsa2048, rsa3072, rsa4096, or ecdsa-p256", algorithm);
        }
    }
}