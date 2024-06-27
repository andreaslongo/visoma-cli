//! # A library crate

// Lints:
#![warn(clippy::pedantic)]
#![warn(deprecated_in_future)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

use reqwest::blocking::Client;
use reqwest::header;
use std::collections::HashMap;
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
    if config.dry_run {
        create_new_ticket_dry_run(config);
    } else {
        create_new_ticket(config)?;
    }
    Ok(())
}

fn create_new_ticket_dry_run(config: Config) {
    let Config {
        server,
        user,
        title,
        description,
        customer_id,
        address_id,
        ..
    } = config;

    println!("Dry run, this would be done:");
    println!("  Create new ticket:");
    println!("    Server: {server}");
    println!("    User: {user}");
    println!("    Title: {title}");
    println!("    Description: {description}");
    println!("    Customer ID: {customer_id}");
    println!("    Address ID: {address_id}");
}

fn create_new_ticket(config: Config) -> Result<(), Box<dyn Error>> {
    let Config {
        server,
        user,
        password,
        title,
        description,
        customer_id,
        address_id,
        ..
    } = config;

    let client = build_client(&user, &password)?;

    // Build request for creating a new ticket
    let url = format!("https://{server}/api2/ticket/");
    let mut data = HashMap::new();
    data.insert("Title", title);
    data.insert("Description", description);
    data.insert("CustomerId", customer_id.to_string());
    data.insert("AddressId", address_id.to_string());

    let res = client.post(url).json(&data).send()?;
    Ok(())
}

fn build_client(user: &str, password: &str) -> Result<Client, Box<dyn Error>> {
    let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.3";

    let mut headers = header::HeaderMap::new();

    let user_value = header::HeaderValue::from_str(user)?;
    let mut password_value = header::HeaderValue::from_str(password)?;
    password_value.set_sensitive(true);

    headers.insert("X_VSM_USERNAME", user_value);
    headers.insert("X_VSM_PASSWORD", password_value);

    let client = Client::builder()
        .user_agent(user_agent)
        .default_headers(headers)
        .https_only(true)
        .build()?;

    Ok(client)
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
