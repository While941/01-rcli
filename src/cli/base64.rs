use core::fmt;
use std::str::FromStr;

use crate::cli::verify_file_exists;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum Base64Options {
    #[command(name = "encode", about = "Encode a string to base64")]
    Encode(Base64OptionsEncode),
    #[command(name = "decode", about = "Decode a base64 string")]
    Decode(Base64OptionsDecode),
}

#[derive(Debug, Parser)]
pub struct Base64OptionsEncode {
    #[arg(long,value_parser = verify_file_exists, default_value = "-", help = "- means stdin")]
    pub input: String,
    #[arg(long,value_parser = parse_base64_format,default_value = "UrlSafe",help = "url safe or standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64OptionsDecode {
    #[arg(short,long,value_parser = verify_file_exists,default_value = "-", help = "- means stdin")]
    pub input: String,
    #[arg(long,value_parser = parse_base64_format,default_value = "UrlSafe",help = "url safe or standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser, Clone, Copy)]
pub enum Base64Format {
    UrlSafe,
    Standard,
}

fn parse_base64_format(s: &str) -> Result<Base64Format, String> {
    match s {
        "UrlSafe" => Ok(Base64Format::UrlSafe),
        "Standard" => Ok(Base64Format::Standard),
        _ => Err("invalid format".to_string()),
    }
}

impl FromStr for Base64Format {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UrlSafe" => Ok(Base64Format::UrlSafe),
            "Standard" => Ok(Base64Format::Standard),
            _ => Err("invalid format".to_string()),
        }
    }
}

impl From<Base64Format> for String {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::UrlSafe => "url safe".to_string(),
            Base64Format::Standard => "standard".to_string(),
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Base64Format::UrlSafe => write!(f, "url safe"),
            Base64Format::Standard => write!(f, "standard"),
        }
    }
}
