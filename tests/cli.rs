use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn generate_fibonacci_sequence() {
    let mut cmd = Command::cargo_bin("rust-starter").unwrap();
    cmd.arg("5")
        .assert()
        .success()
        .stdout(predicate::str::contains("1 1 2 3 5"));
}
