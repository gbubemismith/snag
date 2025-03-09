use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("snag").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("snag").unwrap();
    cmd.arg("-u https://wishnow.io").assert().success();
}
