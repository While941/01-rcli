use clap::Parser;
use rcli::{process_csv, Opts, Subcommand};

// rcli csv -input input.csv -output output.json -delimiter , -header

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    // match opts.cmd {
    //     Subcommand::Csv(csv_opts) => println!("{:?}", csv_opts),
    // }
    match opts.cmd {
        Subcommand::Csv(opts) => process_csv(&opts.input, &opts.output)?,
    }

    Ok(())
}
