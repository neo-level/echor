use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

/// Test the `echor` command with the given arguments.
/// args -> a slice of arguments to pass to the command
/// file -> the file to be used as input
/// expected -> the expected output
fn run(args: &[&str], file: &str) -> TestResult {
    let expected = fs::read_to_string(file)?;
    Command::cargo_bin("echor")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

/// Expected to fail because no arguments are given.
#[test]
fn dies_no_args() {
    Command::cargo_bin("echor")
        .unwrap()
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}

/// Expected to succeed because the required argument is given.
#[test]
fn runs() {
    Command::cargo_bin("echor")
        .unwrap()
        .arg("hello")
        .assert()
        .success();
}

/// Compare the output with the expected output.
#[test]
fn hello1() -> TestResult {
    run(&["hello there"], "tests/expected/hello1.txt")
}

/// Compare the output with the expected output.
#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

/// Compare the output with the expected output.
#[test]
fn hello1_no_newline() -> TestResult {
    run(&["Hello   there", "-n"], "tests/expected/hello1.n.txt")
}

/// Compare the output with the expected output.
#[test]
fn hello2_no_newline() -> TestResult {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}

