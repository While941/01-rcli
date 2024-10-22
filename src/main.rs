use clap::Parser;
use rcli::{gen_pass, process_csv, Opts, Subcommand};

// rcli csv -input input.csv -output output.json -delimiter , -header

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    // match opts.cmd {
    //     Subcommand::Csv(csv_opts) => println!("{:?}", csv_opts),
    // }

    match opts.cmd {
        Subcommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)
        }?,
        Subcommand::Generatepass(password) => {
            gen_pass(
                password.length,
                password.uppercase,
                password.lowercase,
                password.number,
                password.symbol,
            )
        }?,
    }

    Ok(())
}
