use assert_cmd::Command;

fn cmd() -> Command {
    Command::cargo_bin("aggregate").unwrap()
}

#[test]
fn help() {
    cmd().arg("--help").assert().success();
}

#[test]
fn no_args() {
    cmd().assert().code(2);
}

#[test]
fn empty_stdin() {
    cmd().arg("-a=min").assert().success().stdout("\n");
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
            .success()
            .stdout(format!("{}\n", result));
    }
}

#[test]
fn custom_column() {
    let column_sums = [38, 50, 32, 0];

    for (index, sum) in column_sums.iter().enumerate() {
        cmd()
            .args(&["--aggr=sum", &format!("--column-index={}", index)])
            .pipe_stdin("tests/resources/multi-column.tsv")
            .unwrap()
            .assert()
            .success()
            .stdout(format!("{}\n", sum));
    }
}
