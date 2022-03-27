use clap::Parser;
use process::args::ProcessArgs;

fn main() {
    let args = ProcessArgs::parse();
    println!("{:?}", args);
}
