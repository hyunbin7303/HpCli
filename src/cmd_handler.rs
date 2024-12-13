

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
            if encrypting.algorithm.as_deref().unwrap() == "sha256".to_string() {
                match hash_helper::encrypt_sha256(inputStr) {
                    Ok(hash_value) =>println!("Result: {}", hash_value),
                    Err(e) => panic!("Problem getting encrypting the file : {e:?}"),
                };
            }
            else if encrypting.algorithm.as_deref().unwrap() == "md5".to_string() {
                match hash_helper::encrypt_md5(inputStr) {
                    Ok(hash_value) =>println!("Result: {}", hash_value),
                    Err(e) => panic!("Problem getting encrypting the file : {e:?}"),
                };
            }
            else {
                println!("Invalid hash algorithm.")
            }
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