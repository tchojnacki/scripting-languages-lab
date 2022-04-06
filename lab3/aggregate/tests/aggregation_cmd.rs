use assert_cmd::Command;

fn cmd() -> Command {
    Command::cargo_bin("aggregate").unwrap()
}

#[test]
fn help() {
    cmd().arg("--help").assert().code(0);
}

#[test]
fn no_args() {
    cmd().assert().code(2);
}

#[test]
fn empty_stdin() {
    cmd().arg("-a=min").assert().code(0).stdout("\n");
}

#[test]
fn aggregators() {
    let results = [
        ("avg", "55.76470588235294"),
        ("count", "51"),
        ("first", "50"),
        ("last", "99"),
        ("max", "100"),
        ("min", "3"),
        ("range", "97"),
        ("sum", "2844"),
    ];

    for (aggregation, result) in results {
        cmd()
            .arg(format!("-a={}", aggregation))
            .pipe_stdin("tests/resources/numbers.txt")
            .unwrap()
            .assert()
            .code(0)
            .stdout(format!("{}\n", result));
    }
}
