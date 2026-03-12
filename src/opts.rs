use clap::Parser;
use std::string::String;

#[derive(Debug, Parser)]
#[command(name = "my-cli", about = "我的cli命令行程序")]
pub struct Opts {
    #[command(subcommand)]
    cmd: Subcommand,
}

#[derive(Debug, Parser)]
enum Subcommand {
    #[command(name = "csv", about = "convert csv to json")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(long, default_value = "input.csv", value_parser = verify_input)]
    pub input: String,
    #[arg(long, default_value = "output.csv")]
    pub output: String,
    #[arg(long, default_value_t = false)]
    pub header: bool,
    #[arg(long, default_value_t = ' ')]
    pub delimiter: char,
}

pub fn verify_input(filename: &str) -> std::result::Result<String, String> {
    // if(std::path::Path::new(filename).exists()) {
    //     Ok("ok".to_string())
    // }else {
    //     Err("err".to_string())
    // }
    Ok(filename.to_string())
}
