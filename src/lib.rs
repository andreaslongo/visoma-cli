//! # A library crate

// Lints:
#![warn(clippy::pedantic)]
#![warn(deprecated_in_future)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

use reqwest::blocking::Client;
use reqwest::header;
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
        let mut headers = header::HeaderMap::new();
        headers.insert("VMX-USER", header::HeaderValue::from_str(&user)?);
        let mut auth_value = header::HeaderValue::from_str(&password)?;
        auth_value.set_sensitive(true);
        headers.insert("VMX-PASSWORD", auth_value);
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.3")
            .default_headers(headers)
            .https_only(true)
            .build()?;

        // https://stackoverflow.com/questions/5725430/http-test-server-accepting-get-post-requests
        let res = client.get("https://httpbin.org/anything").send()?;
        dbg!(&res);
        dbg!(&res.text()?);
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
