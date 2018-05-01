extern crate openssl;
extern crate rand;

use openssl::symm::{encrypt, Cipher};
use rand::Rng;

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

    let ciphertext = encrypt(cipher, key, Some(iv.as_bytes()), a);

    println!("{:?}", ciphertext);
}
