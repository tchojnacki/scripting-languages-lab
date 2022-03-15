use itertools::Itertools;
use std::{
    cmp,
    collections::HashMap,
    env,
    io::{self, BufRead},
    process,
};

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let mut arg_counts = args
        .iter()
        .map(|arg| (arg.as_str(), 0))
        .collect::<HashMap<&str, usize>>();

    for line in io::stdin().lock().lines().flatten() {
        for (key, value) in arg_counts.iter_mut() {
            *value += line.matches(key).count();
        }
    }

    let mut args_by_count = arg_counts
        .iter()
        .sorted_by_key(|(_, &count)| cmp::Reverse(count));

    let most_common_arg = args_by_count.next();
    if matches!(most_common_arg, Some((_, &count)) if count > 0) {
        let most_common_arg = most_common_arg.unwrap().0;
        let code = args
            .iter()
            .position(|arg| arg == most_common_arg)
            .map(|idx| idx + 1)
            .unwrap_or(0) as i32;

        process::exit(code);
    }
}
