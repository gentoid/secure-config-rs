extern crate base64;
extern crate failure;
extern crate openssl;
extern crate rand;

use failure::Error;
use openssl::symm::{encrypt, Cipher};
use rand::Rng;

pub struct Encode {
    pub key: String,
    pub string: String,
}

pub fn encode(to_encode: Encode) -> Result<String, Error> {
    let cipher = Cipher::aes_256_cfb1();

    let mut iv = String::new();
    for c in rand::thread_rng().gen_ascii_chars().take(16) {
        iv.push(c);
    }

    let ciphertext = encrypt(cipher, to_encode.key.as_bytes(), Some(iv.as_bytes()), to_encode.string.as_bytes()).unwrap();
    let encoded = base64::encode(&ciphertext);
    let result = format!("ENC({})::IV({})", encoded, base64::encode(&iv));
    Ok(result)
}
