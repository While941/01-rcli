mod base64;
mod csv;
mod genpass;
mod text;

use std::path::Path;

use clap::Parser;

pub use self::{
    base64::{Base64Format, Base64Options},
    csv::CsvOpts,
    csv::OutputFormat,
    genpass::GeneratepassOpts,
    text::TextSubCommand,
};

#[derive(Debug, Parser)]
#[command(name = "rcli", version,author,about,long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "csv command")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "generatepass command")]
    Generatepass(GeneratepassOpts),
    #[command(subcommand)]
    Base64(Base64Options),
    #[command(subcommand)]
    Text(text::TextSubCommand),
}

fn verify_file_exists(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exits or stdin is empty")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_file() {
        // assert_eq!(verify_file_exists("tests/data.csv"), Ok("tests/data.csv".into()));
        assert_eq!(verify_file_exists("-"), Ok("-".into()));
    }
}
