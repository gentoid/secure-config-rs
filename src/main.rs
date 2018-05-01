extern crate base64;
extern crate clap;
extern crate openssl;
extern crate rand;

use std::env;
use std::str;
use clap::{Arg, App, SubCommand};
use openssl::symm::{encrypt, Cipher, decrypt};
use rand::Rng;

fn main() {
    let matches = App::new("Something encoding/decoding")
        .version("0.1.0")
        .about("This tool allows to encode and decode strings with OpenSSL AES256 CFB1 algorithm")
        .subcommand(SubCommand::with_name("encode")
                 .about("encodes givem string with random IV and returns a string ready to be copy&pasted into a config")
                 .arg(Arg::with_name("string")
                        .required(true)
                        .help("A string to be encoded")))
        .subcommand(SubCommand::with_name("decode")
                 .about("decodes encoded string")
                 .arg(Arg::with_name("string")
                        .required(true)
                        .help("A string to be decoded. It should be in the format of 'ENC(...)::IV(...)', otherwise it won't be decoded")))
        .get_matches();

    let string_to_encode = matches.subcommand_matches("encode").unwrap().value_of("string").unwrap();
    let string_to_decode = matches.subcommand_matches("decode").unwrap().value_of("string").unwrap();

    let key = env::var("SECURE_CONFIG_KEY").unwrap();
    let key = key.as_bytes();

    let cipher = Cipher::aes_256_cfb1();

    let mut iv = String::new();
    for c in rand::thread_rng().gen_ascii_chars().take(16) {
        iv.push(c);
    }

    let ciphertext = encrypt(cipher, key, Some(iv.as_bytes()), string_to_encode.as_bytes()).unwrap();

    let encoded = base64::encode(&ciphertext);
    let decoded = base64::decode(&encoded).unwrap();

    println!("ENC({})::IV({})", encoded, base64::encode(&iv));
    println!("{:?}", decoded);

    let decrypted = decrypt(cipher, key, Some(iv.as_bytes()), &decoded);

    println!("decrypted {:?}", str::from_utf8(&decrypted.unwrap()).unwrap());
}
