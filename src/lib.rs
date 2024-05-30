use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Config {
    #[arg(default_value_t = String::from("File1"))]
    pub file1: String,
    #[arg(default_value_t = String::from("File2"))]
    pub file2: String,
}
