extern crate base64;
extern crate failure;
extern crate openssl;
extern crate rand;

use std::env;
use std::str;

use failure::Error;
use openssl::symm::{encrypt, Cipher, decrypt};
use rand::Rng;

pub struct Encode {
    pub key: String,
    pub string: String,
}

pub struct Decode {
    pub key: String,
    pub string: String,
}

pub fn get_key() -> Result<String, env::VarError> {
    env::var("SECURE_CONFIG_KEY")
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

pub fn decode(to_decode: Decode) -> Result<String, Error> {
    let cipher = Cipher::aes_256_cfb1();

    let parts: Vec<&str> = to_decode.string.as_str().split("::").collect();

    assert!(parts.len() == 2);

    let encoded = parts[0];
    let iv = parts[1];

    assert!(encoded.to_string().starts_with("ENC("));
    assert!(encoded.to_string().ends_with(")"));
    assert!(iv.to_string().starts_with("IV("));
    assert!(iv.to_string().ends_with(")"));

    let encoded = encoded.to_string().replacen("ENC(", "", 1).replacen(")", "", 1);
    let iv = iv.to_string().replacen("IV(", "", 1).replacen(")", "", 1);

    let decoded = base64::decode(&encoded).unwrap();
    let iv = base64::decode(&iv).unwrap();

    let decrypted = decrypt(cipher, to_decode.key.as_bytes(), Some(&iv), &decoded).unwrap();

    Ok(str::from_utf8(&decrypted).unwrap().to_string())
}
