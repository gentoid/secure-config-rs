extern crate base64;
extern crate openssl;
extern crate rand;

use openssl::symm::{encrypt, Cipher, decrypt};
use rand::Rng;
use std::str;

fn main() {
    let a = b"hello world";
    let key = b"000102030405060708090a0b0c0d0e0f"; // .as_bytes();

    // let iv = "000102030405060708090a0b0c0d0e0f".as_bytes();

    let cipher = Cipher::aes_256_cfb1();

    println!("Cipher key len: {:?}", cipher.key_len());
    println!("Cipher IV len: {:?}", cipher.iv_len());

    let mut iv = String::new();
    for c in rand::thread_rng().gen_ascii_chars().take(16) {
        iv.push(c);
    }

    println!("used IV {:?}", iv);

    let ciphertext = encrypt(cipher, key, Some(iv.as_bytes()), a).unwrap();

    let encoded = base64::encode(&ciphertext);
    let decoded = base64::decode(&encoded).unwrap();

    println!("ENC({})::IV({})", encoded, base64::encode(&iv));
    println!("{:?}", decoded);

    let decrypted = decrypt(cipher, key, Some(iv.as_bytes()), &decoded);

    // println!("{:?}", ciphertext);
    println!("decrypted {:?}", str::from_utf8(&decrypted.unwrap()).unwrap());
}
