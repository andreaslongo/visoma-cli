//! # An integration test crate

// Lints:
#![warn(clippy::pedantic)]
#![warn(deprecated_in_future)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn print_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("visoma-cli")?;
    cmd.arg("--help");
    cmd.assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains(
            "A CLI utility for interacting with Visoma",
        ));
    Ok(())
}

#[test]
fn print_version() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("visoma-cli")?;
    cmd.arg("--version");
    cmd.assert()
        .success()
        .code(0)
        .stdout(predicate::str::is_match(r"^visoma-cli \d+\.\d+\.\d+\n$")?);
    Ok(())
}
