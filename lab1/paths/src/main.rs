use clap::StructOpt;
use paths::{traverse, Config};
use std::process;

fn main() {
    let config = Config::parse();

    if let Err(e) = traverse(config) {
        eprintln!("An error occured while reading the file system: {}", e);
        process::exit(1);
    }
}
