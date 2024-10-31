use core::fmt;
use std::str::FromStr;

use crate::cli::verify_file_exists;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(about = "")]
    Sign(TextSignOpts),
    #[command(about = "")]
    Verify(TextVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short,long,value_parser = verify_file_exists, default_value = "-")]
    pub input: String,
    #[arg(long,value_parser = verify_file_exists, default_value = "-")]
    pub key: String,
    #[arg(short, long, default_value = "Blake3")]
    pub format: String,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short,long,value_parser = verify_file_exists, default_value = "-")]
    pub input: String,
    #[arg(long,value_parser = verify_file_exists, default_value = "-")]
    pub key: String,
    #[arg(short,long,value_parser = parse_format , default_value = "Blake3")]
    pub format: TextSignFormat,
    #[arg(short, long)]
    pub sign: String,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

impl FromStr for TextSignFormat {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Blake3" => Ok(TextSignFormat::Blake3),
            "Ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err("invalid format".to_string()),
        }
    }
}

impl From<TextSignFormat> for String {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3".to_string(),
            TextSignFormat::Ed25519 => "ed25519".to_string(),
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextSignFormat::Blake3 => write!(f, "blake3"),
            TextSignFormat::Ed25519 => write!(f, "ed25519"),
        }
    }
}

fn parse_format(s: &str) -> Result<TextSignFormat, String> {
    match s {
        "Blake3" => Ok(TextSignFormat::Blake3),
        "Ed25519" => Ok(TextSignFormat::Ed25519),
        _ => Err("invalid format".to_string()),
    }
}
