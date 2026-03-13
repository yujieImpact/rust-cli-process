use clap::Parser;
use my_cli::{process_csv, Opts, Subcommand};

fn main() -> anyhow::Result<()> {
    let opt = Opts::parse();

    ;
    match opt.cmd {
        Subcommand::Csv(x) => process_csv(&x.input, &x.output),
    }

}
