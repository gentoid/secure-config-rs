extern crate secure_config;

extern crate clap;

use clap::{Arg, App, SubCommand};
use secure_config::{encode, Encode, get_key, decode, Decode};

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

    if let Some(sumbcommand) = matches.subcommand_matches("encode") {
        let key = get_key().unwrap();
        let string_to_encode = sumbcommand.value_of("string").unwrap();
        println!("{:?}", encode(Encode { key: key, string: string_to_encode.to_string() }).unwrap());
    }

    if let Some(sumbcommand) = matches.subcommand_matches("decode") {
        let key = get_key().unwrap();
        let string_to_encode = sumbcommand.value_of("string").unwrap();
        println!("{:?}", decode(Decode { key: key, string: string_to_encode.to_string() }).unwrap());
    }
}
