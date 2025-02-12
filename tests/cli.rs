use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn unexpected_argument() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ata")?;

    cmd.arg("foobar");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("unexpected argument"));

    Ok(())
}

#[test]
fn help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ata")?;

    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Usage: ata"));

    Ok(())
}
