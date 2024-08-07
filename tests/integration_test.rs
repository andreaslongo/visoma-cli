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
use visoma_cli::AppError;

#[test]
fn help() -> Result<(), AppError> {
    let mut cmd = Command::cargo_bin("visoma-cli")?;
    cmd.arg("--help");
    cmd.assert()
        .success()
        .code(0)
        .stderr(predicate::str::is_empty())
        .stdout(predicate::str::contains(
            "A CLI utility for interacting with Visoma",
        ));
    Ok(())
}

#[test]
fn version() -> Result<(), AppError> {
    let mut cmd = Command::cargo_bin("visoma-cli")?;
    cmd.arg("--version");
    cmd.assert()
        .success()
        .code(0)
        .stderr(predicate::str::is_empty())
        .stdout(predicate::str::is_match(r"^visoma-cli \d+\.\d+\.\d+\n$")?);
    Ok(())
}

#[test]
fn ticket_new_syntax() -> Result<(), AppError> {
    let mut cmd = Command::cargo_bin("visoma-cli")?;
    cmd.arg("ticket").arg("new").arg("--help");
    cmd.assert()
        .success()
        .code(0)
        .stderr(predicate::str::is_empty())
        .stdout(predicate::str::contains("Usage: visoma-cli ticket new [OPTIONS] --server <SERVER> --user <USER> --password <PASSWORD> --title <TITLE> --description <DESCRIPTION> --customer-id <CUSTOMER_ID> --address-id <ADDRESS_ID>"));
    Ok(())
}

#[test]
fn ticket_new_dry_run() -> Result<(), AppError> {
    let mut cmd = Command::cargo_bin("visoma-cli")?;
    cmd.args([
        "ticket",
        "new",
        "--dry-run",
        "--server",
        "example.com",
        "--user",
        "test",
        "--password",
        "test123",
        "--title",
        "Test Ticket",
        "--description",
        "A new ticket for testing",
        "--customer-id",
        "1",
        "--address-id",
        "2",
    ]);
    cmd.assert()
        .success()
        .code(0)
        .stderr(predicate::str::is_empty())
        .stdout(predicate::str::contains("Dry run"))
        .stdout(predicate::str::contains("Create new ticket"))
        .stdout(predicate::str::contains("Server: example.com"))
        .stdout(predicate::str::contains("User: test"))
        .stdout(predicate::str::contains("Title: Test Ticket"))
        .stdout(predicate::str::contains(
            "Description: A new ticket for testing",
        ))
        .stdout(predicate::str::contains("Customer ID: 1"))
        .stdout(predicate::str::contains("Address ID: 2"));
    Ok(())
}

#[test]
fn ticket_new_dry_run_with_optional_args() -> Result<(), AppError> {
    let mut cmd = Command::cargo_bin("visoma-cli")?;
    cmd.args([
        "ticket",
        "new",
        "--dry-run",
        "--server",
        "example.com",
        "--user",
        "test",
        "--password",
        "test123",
        "--title",
        "Test Ticket",
        "--description",
        "A new ticket for testing",
        "--customer-id",
        "1",
        "--address-id",
        "2",
        "--arranger-id",
        "3",
    ]);
    cmd.assert()
        .success()
        .code(0)
        .stderr(predicate::str::is_empty())
        .stdout(predicate::str::contains("Dry run"))
        .stdout(predicate::str::contains("Create new ticket"))
        .stdout(predicate::str::contains("Server: example.com"))
        .stdout(predicate::str::contains("User: test"))
        .stdout(predicate::str::contains("Title: Test Ticket"))
        .stdout(predicate::str::contains(
            "Description: A new ticket for testing",
        ))
        .stdout(predicate::str::contains("Customer ID: 1"))
        .stdout(predicate::str::contains("Address ID: 2"))
        .stdout(predicate::str::contains("Arranger ID: 3"));
    Ok(())
}
#[test]
fn ticket_new() -> Result<(), AppError> {
    let mut cmd = Command::cargo_bin("visoma-cli")?;
    cmd.args([
        "ticket",
        "new",
        "--server",
        "httpbin.org/anything",
        "--user",
        "test",
        "--password",
        "test123",
        "--title",
        "Test Ticket",
        "--description",
        "A new ticket for testing",
        "--customer-id",
        "1",
        "--address-id",
        "2",
    ]);
    cmd.assert()
        .success()
        .code(0)
        .stderr(predicate::str::is_empty())
        .stdout(predicate::str::is_empty());
    Ok(())
}

#[test]
fn ticket_new_with_optional_args() -> Result<(), AppError> {
    let mut cmd = Command::cargo_bin("visoma-cli")?;
    cmd.args([
        "ticket",
        "new",
        "--server",
        "httpbin.org/anything",
        "--user",
        "test",
        "--password",
        "test123",
        "--title",
        "Test Ticket",
        "--description",
        "A new ticket for testing",
        "--customer-id",
        "1",
        "--address-id",
        "2",
        "--arranger-id",
        "3",
    ]);
    cmd.assert()
        .success()
        .code(0)
        .stderr(predicate::str::is_empty())
        .stdout(predicate::str::is_empty());
    Ok(())
}

#[test]
fn ticket_new_fails_if_server_does_not_exist() -> Result<(), AppError> {
    let mut cmd = Command::cargo_bin("visoma-cli")?;
    cmd.args([
        "ticket",
        "new",
        "--server",
        "does.not.exist",
        "--user",
        "test",
        "--password",
        "test123",
        "--title",
        "Test Ticket",
        "--description",
        "A new ticket for testing",
        "--customer-id",
        "1",
        "--address-id",
        "2",
        "--arranger-id",
        "3",
    ]);
    cmd.assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains(
            "error sending request for url (https://does.not.exist/api2/ticket/)",
        ))
        .stdout(predicate::str::is_empty());
    Ok(())
}
