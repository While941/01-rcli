use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE},
    Engine as _,
};

use crate::cli::Base64Format;
use std::{fs::File, io::Read};

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let encode = match format {
        Base64Format::Standard => STANDARD.encode(&buffer),
        Base64Format::UrlSafe => URL_SAFE.encode(&buffer),
    }; // let encode = URL_SAFE.encode(buffer);
    println!("{}", encode);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let decode = match format {
        Base64Format::Standard => STANDARD.decode(&buffer),
        Base64Format::UrlSafe => URL_SAFE.decode(&buffer),
    }; // let encode = URL_SAFE.encode(buffer);

    let decode = String::from_utf8(decode?)?;
    println!("{}", decode);

    Ok(())
}

//  wrong code
// pub fn process_decode(input: &str) -> anyhow::Result<()> {
// let decode = URL_SAFE.decode(input);
// let decode = String::from_utf8(decode);
// Ok(())
// }
