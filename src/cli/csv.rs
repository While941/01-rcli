use core::fmt;
use std::str::FromStr;

use crate::cli::verify_file_exists;
use clap::Parser;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}
#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_file_exists)]
    pub input: String,

    #[arg(short, long)]
    // default value 会调用 into() 函数 使得 &str -> String 类型转换 ,default_value_t 返回的是一个&str类型无法直接赋值给String类型
    pub output: Option<String>,

    #[arg(short, long, value_parser = parse_format , default_value = "json")]
    pub format: OutputFormat,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn parse_format(s: &str) -> Result<OutputFormat, anyhow::Error> {
    s.parse::<OutputFormat>()
}

impl From<OutputFormat> for &'static str {
    fn from(f: OutputFormat) -> Self {
        match f {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self)) // 实现了FromStr和From trait的OutputFormat类型可以调用Into::into()函数转换为&str
    }
}
