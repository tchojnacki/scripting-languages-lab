use std::env;

fn main() {
    let env_vars = env::vars()
        .map(|(key, value)| format!("{key}={value}"))
        .collect::<Vec<_>>()
        .join(" ");

    let params = env::args().skip(1).collect::<Vec<_>>().join(" ");

    println!("{env_vars}\n{params}");
}
