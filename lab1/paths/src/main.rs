use clap::StructOpt;
use paths::{traverse, Config};

fn main() {
    let config = Config::parse();
    traverse(&config);
}
