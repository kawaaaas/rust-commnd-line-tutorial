use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    Command::cargo_bin("echo_02")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

fn run(outfile: &str, args: &[&str]) -> TestResult {
    let expected = fs::read(outfile)?;
    Command::cargo_bin("echo_02")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    run("tests/expected/hello1.txt", &["Hello there"])
}
#[test]
fn hello1n() -> TestResult {
    run("tests/expected/hello1.n.txt", &["Hello there", "-n"])
}
#[test]
fn hello2() -> TestResult {
    run("tests/expected/hello2.txt", &["Hello", "there"])
}
#[test]
fn hello2n() -> TestResult {
    run("tests/expected/hello2.n.txt", &["Hello", "there", "-n"])
}
