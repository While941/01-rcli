use clap::Parser;
#[derive(Debug, Parser)]
#[command(name = "rcli", version,author,about,long_about=None)]
struct Opts {
    #[command(subcommand)]
    cmd: Subcommand,
}

#[derive(Debug, Parser)]
enum Subcommand {
    #[command(name = "csv", about = "csv command")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long)]
    input: String,

    #[arg(short, long, default_value = "output.json")]
    output: String,
    #[arg(short, long)]
    delimiter: char,
    #[arg(short, long, default_value_t = true)]
    header: bool,
}

fn main() {
    println!("Hello, Rust!");
}
