

use core::panic;

use crate::hash_helper;
use crate::commands::{Encrypting, StringSearch};


pub fn crypto_handler(encrypting: &Encrypting) {
    let result = match encrypting.input_type.as_deref() {
        Some("string") => {
            let inputStr = match encrypting.input_string.as_deref() {
                Some(input) => input,
                None => {
                    return
                }
            };

            // let create: Result<String> = Result.
            let result = match encrypting.algorithm.as_ref().unwrap().to_lowercase().as_str() {
                "sha256" => hash_helper::encrypt_sha256(inputStr),
                "sha512" => hash_helper::encrypt_sha512(inputStr),
                "md5" => hash_helper::encrypt_md5(inputStr),
                _ => return, // Ideally we need to return Result<String>
            };
            println!("Checking out");
            if result.is_err() {

                return
            }

            match result {
                Ok(hash_value) => println!("Result: {}", hash_value),
                Err(e) => panic!("Problem getting encrypting the file : {e:?}"),
            };


            // else if encrypting.algorithm.as_deref()
            // let result = match hash_helper::encrypt_sha512(inputStr) {
            //     Ok(hash_value) => println!("Result: {}", hash_value),
            //     Err(e) => panic!("Problem getting encrypting the file : {e:?}"),
            // };

            return
        }
        Some("file") => {

            true
        }
        _ => {
            false
        }
    };
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