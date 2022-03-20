use clap::StructOpt;
use head_tail::{
    args::{CommonArgs, HeadArgs},
    shared::handle_results,
};
use std::io::{self, BufRead};

fn main() {
    let args = HeadArgs::parse();

    let lines = io::stdin()
        .lock()
        .lines()
        .flatten()
        .take(args.expected_lines())
        .collect::<Vec<_>>();

    handle_results(lines, &args);
}
