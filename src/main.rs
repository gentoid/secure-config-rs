extern crate openssl;

use openssl::symm::{encrypt, Cipher};

fn main() {
    let a = b"hello world";
    let key = b"000102030405060708090a0b0c0d0e0f"; // .as_bytes();

    // let iv = "000102030405060708090a0b0c0d0e0f".as_bytes();

    let cipher = Cipher::aes_256_cfb1();

    println!("Cipher key len: {:?}", cipher.key_len());
    println!("Cipher IV len: {:?}", cipher.iv_len());

    let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";

    let ciphertext = encrypt(cipher, key, Some(iv), a);

    println!("{:?}", ciphertext);
}
