use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn dies_no_args() {
    Command::cargo_bin("echor")
        .unwrap()
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}
