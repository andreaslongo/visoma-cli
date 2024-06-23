//! # A library crate

// Lints:
#![warn(clippy::pedantic)]
#![warn(deprecated_in_future)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

use std::error::Error;

/// The configuration for the program
#[derive(Debug)]
pub struct Config {
    dry_run: bool,
    server: String,
    user: String,
    password: String,
    title: String,
    description: String,
    customer_id: usize,
    address_id: usize,
}

impl Config {
    /// Returns a new `Config`
    #[must_use]
    pub fn new(
        dry_run: bool,
        server: String,
        user: String,
        password: String,
        title: String,
        description: String,
        customer_id: usize,
        address_id: usize,
    ) -> Self {
        Self {
            dry_run,
            server,
            user,
            password,
            title,
            description,
            customer_id,
            address_id,
        }
    }
}

/// Performs the main actions
///
/// # Errors
///
/// Can fail.
pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    if config.dry_run {
        println!("Dry run, this would be done:");
        println!("  Create new ticket:");
        println!("    Server: {0}", config.server);
        println!("    User: {0}", config.user);
        println!("    Ticket: {0}", config.title);
        println!("    Description: {0}", config.description);
        println!("    Customer ID: {0}", config.customer_id);
        println!("    Address ID: {0}", config.address_id);
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
