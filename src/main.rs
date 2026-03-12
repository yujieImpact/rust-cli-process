use clap::Parser;
use my_cli::Opts;

fn main() {
    let opt = Opts::parse();
    println!("{:?}", opt);
}
