extern crate secure_config;

extern crate base64;
extern crate clap;
extern crate openssl;
extern crate rand;

use std::env;
// use std::str;
use clap::{Arg, App, SubCommand};
use secure_config::{encode, Encode};
// use openssl::symm::{encrypt, Cipher, decrypt};
// use rand::Rng;

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

    let key = env::var("SECURE_CONFIG_KEY").unwrap();

    if let Some(sumbcommand) = matches.subcommand_matches("encode") {
        let string_to_encode = sumbcommand.value_of("string").unwrap();
        println!("{:?}", encode(Encode { key: key, string: string_to_encode.to_string() }).unwrap());
    }


    // let string_to_decode = matches.subcommand_matches("decode").unwrap().value_of("string").unwrap();


    // let cipher = Cipher::aes_256_cfb1();



    // let decoded = base64::decode(&encoded).unwrap();

    // println!("{:?}", decoded);

    // let decrypted = decrypt(cipher, key, Some(iv.as_bytes()), &decoded);

    // println!("decrypted {:?}", str::from_utf8(&decrypted.unwrap()).unwrap());
}
