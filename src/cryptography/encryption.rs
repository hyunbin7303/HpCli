use std::{error::Error};
use std::io::{Read, Write};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use orion::hazardous::{
    aead::xchacha20poly1305::SecretKey as XSecretKey,
    stream::xchacha20::XCHACHA_NONCESIZE
};
use orion::hazardous::stream::chacha20::CHACHA_KEYSIZE;
use orion::kdf::{derive_key, Password, Salt};
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









