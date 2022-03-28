use crate::args::ProcessArgs;
use std::io::{self, BufRead};

#[repr(i32)]
pub enum ProcessResult {
    Success = 0,
    NothingToPrint = 1,
    EmptyStdin = 2,
}

fn extract_substring(line: String, from_start: usize, from_end: usize) -> String {
    line.chars()
        .skip(from_start)
        .take(line.len().saturating_sub(from_start + from_end))
        .collect()
}

fn split_project_join(
    line: String,
    old_separator: &str,
    new_separator: &str,
    filter: &Option<Vec<usize>>,
) -> String {
    let split_line = line.split(old_separator);

    let line_segments = if let Some(columns) = filter {
        split_line
            .enumerate()
            .filter_map(|(i, v)| if columns.contains(&i) { Some(v) } else { None })
            .collect::<Vec<_>>()
    } else {
        split_line.collect::<Vec<_>>()
    };

    line_segments.join(new_separator)
}

fn should_be_printed(line: &str, select: &Option<String>) -> bool {
    if let Some(pattern) = select {
        line.contains(pattern)
    } else {
        true
    }
}

pub fn process_lines(args: &ProcessArgs) -> ProcessResult {
    let mut stdin_empty = true;
    let mut stdout_empty = true;

    for line in io::stdin().lock().lines().flatten() {
        stdin_empty = false;

        let line = extract_substring(line, args.ignorefirst, args.ignorelast);
        let line = split_project_join(line, &args.delimiter, &args.separator, &args.project);

        if should_be_printed(&line, &args.select) {
            stdout_empty = false;
            println!("{}", line);
        }
    }

    match (stdin_empty, stdout_empty) {
        (true, _) => ProcessResult::EmptyStdin,
        (false, true) => ProcessResult::NothingToPrint,
        _ => ProcessResult::Success,
    }
}
