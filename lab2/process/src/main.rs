use process::{args::parse_args_or_exit, run::process_lines};

fn main() {
    let args = parse_args_or_exit();
    let result = process_lines(&args);
    std::process::exit(result as i32);
}
