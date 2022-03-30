use assert_cmd::Command;

fn cmd() -> Command {
    Command::cargo_bin("process").unwrap()
}

#[test]
fn help() {
    cmd().arg("--help").assert().code(64);
}

#[test]
fn empty_stdin() {
    cmd().assert().code(2);
}

#[test]
fn empty_stdout() {
    cmd().arg("--select=a").write_stdin("b").assert().code(1);
}

#[test]
fn simple_csv() {
    cmd()
        .args(["--delimiter=,", "--project=1", "--select=07"])
        .pipe_stdin("tests/resources/simple.csv")
        .unwrap()
        .assert()
        .success()
        .stdout("2070\n5079\n");
}
