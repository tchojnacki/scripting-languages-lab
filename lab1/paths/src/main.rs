use paths::{traverse, Config};
use std::{env, process};

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();

    let config = Config::from_args(&args).unwrap_or_else(|err| {
        use paths::ArgumentParsingError::*;
        match err {
            UnknownArgument(arg) => eprintln!("Unknown argument: {arg}!"),
            NoSortingType => eprintln!("No sorting type passed to --sort!"),
            BadSortingType(sorting_type) => eprintln!("Unknown sorting type: {sorting_type}!"),
        };

        process::exit(1);
    });

    if let Err(e) = traverse(config) {
        eprintln!("An error occured while reading the file system: {}", e);
        process::exit(1);
    }
}
