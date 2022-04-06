use aggregate::{args::AggregateArgs, run};
use clap::StructOpt;
use std::process;

fn main() {
    let mut args = AggregateArgs::parse();

    if let Err(err) = run::parse_data_labels(&mut args) {
        eprintln!("Error: {}", err);
        process::exit(1);
    }

    run::process(&args);
}
