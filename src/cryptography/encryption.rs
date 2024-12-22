use std::{error::Error};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::fs;
use std::io::{Read, Write};

use orion::hazardous::{
    aead::xchacha20poly1305::{seal, open, Nonce, SecretKey as XSecretKey},
    mac::poly1305::POLY1305_OUTSIZE,
    stream::xchacha20::XCHACHA_NONCESIZE
};
use orion::hazardous::stream::chacha20::CHACHA_KEYSIZE;
use orion::kdf::{derive_key, Password, Salt};
use orion::errors::UnknownCryptoError;
use rand::prelude::*;


#[derive(Debug)]
pub struct CryptoError;
impl Error for CryptoError {}
impl Display for CryptoError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!( f, "Crypto failure" )
    }
}

const NONCE_PLUS_AD_SIZE: usize = XCHACHA_NONCESIZE + 32;

fn split_encrypted( cipher_text: &[u8] ) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    return (
        cipher_text[..XCHACHA_NONCESIZE].to_vec(),
        cipher_text[XCHACHA_NONCESIZE..NONCE_PLUS_AD_SIZE].to_vec(),
        cipher_text[NONCE_PLUS_AD_SIZE..].to_vec(),
    )
}

fn get_random( dest: &mut [u8]) {
    let mut rng = rand::thread_rng();
    rng.fill( dest );
}

fn nonce() -> Vec<u8> {
    let mut randoms: [u8; 24] = [0; 24];
    get_random( &mut randoms );
    return randoms.to_vec();
}

fn auth_tag() -> Vec<u8> {
    let mut randoms: [u8; 32] = [0; 32];
    get_random( &mut randoms );
    return randoms.to_vec();
}

fn create_key( password: String, nonce: Vec<u8> ) -> XSecretKey {
    let password = Password::from_slice(password.as_bytes()).unwrap();
    let salt = Salt::from_slice(nonce.as_slice()).unwrap();
    let kdf_key = derive_key(&password, &salt, 15, 1024, CHACHA_KEYSIZE as u32).unwrap();
    let key = XSecretKey::from_slice( kdf_key.unprotected_as_bytes() ).unwrap();
    return key;
}









