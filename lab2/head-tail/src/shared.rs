use std::process;

use crate::args::CommonArgs;

pub fn handle_results(lines: Vec<String>, args: &impl CommonArgs) {
    for line in &lines {
        println!("{}", line);
    }

    if lines.len() < args.expected_lines() {
        if !args.error_suppression() {
            eprintln!(
                "Zabrakło {} linii do wypisania.",
                args.expected_lines() - lines.len()
            );
        }

        process::exit(2);
    }
}
