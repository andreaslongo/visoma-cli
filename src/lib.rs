//! # A library crate

// Lints:
#![warn(clippy::pedantic)]
#![warn(deprecated_in_future)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

use std::error::Error;

/// The configuration for the program
///
/// Fields are documented in main.rs > CLI Args
#[allow(missing_docs)]
#[derive(Debug)]
pub struct Config {
    pub dry_run: bool,
    pub server: String,
    pub user: String,
    pub password: String,
    pub title: String,
    pub description: String,
    pub customer_id: usize,
    pub address_id: usize,
}

/// Performs the main actions
///
/// # Errors
///
/// Can fail.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let Config {
        dry_run,
        server,
        user,
        password,
        title,
        description,
        customer_id,
        address_id,
    } = config;

    if dry_run {
        println!("Dry run, this would be done:");
        println!("  Create new ticket:");
        println!("    Server: {server}");
        println!("    User: {user}");
        println!("    Ticket: {title}");
        println!("    Description: {description}");
        println!("    Customer ID: {customer_id}");
        println!("    Address ID: {address_id}");
    } else {
        println!("Creating new ticket");
    }
    Ok(())
}

/// Calculates `left` + `right` and returns the result.
///
/// # Examples
///
/// ```
/// assert_eq!(visoma_cli::add(1, 2), 3);
/// assert_eq!(visoma_cli::add(5, 5), 10);
/// ```
#[must_use]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
