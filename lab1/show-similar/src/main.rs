use itertools::Itertools;
use std::{collections::HashSet, env};

const VAR_SEPARATOR: char = if cfg!(unix) { ':' } else { ';' };

fn main() {
    let params = env::args().skip(1).collect::<Vec<_>>();

    let (matching_vars, unmatched_params) = match_env_vars_to_params(&params);

    for (var, value) in matching_vars.iter().sorted() {
        print_var(&(var, value));
    }

    for param in unmatched_params {
        print_var(&(param, "NONE"));
    }
}

fn match_env_vars_to_params(params: &[String]) -> (HashSet<(String, String)>, Vec<&String>) {
    let mut matching_vars = HashSet::new();
    let mut unmatched_params = Vec::new();

    for param in params {
        let mut vars_containing_param = env::vars()
            .filter(|(key, _)| key.contains(param))
            .peekable();

        if vars_containing_param.peek().is_none() {
            unmatched_params.push(param);
        }

        matching_vars.extend(vars_containing_param);
    }

    (matching_vars, unmatched_params)
}

fn print_var((key, value): &(&str, &str)) {
    let value = if value.contains(VAR_SEPARATOR) {
        format!("\n\t{}", value.split(VAR_SEPARATOR).join("\n\t"))
    } else {
        value.to_string()
    };

    println!("{key} = {value}");
}
