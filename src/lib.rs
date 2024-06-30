//! # A library crate

// Lints:
#![warn(clippy::pedantic)]
#![warn(deprecated_in_future)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

use reqwest::blocking::Client;
use reqwest::header;
use serde::{Deserialize, Serialize};
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
    pub arranger_id: Option<usize>,
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
        arranger_id,
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
    if let Some(id) = arranger_id {
        println!("    Arranger ID: {id}");
    }
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
        arranger_id,
        ..
    } = config;

    let client = build_client(&user, &password)?;

    let res = client
        .post(format!("https://{server}/api2/ticket/"))
        .json(&NewTicketRequest {
            title,
            description,
            customer_id,
            address_id,
        })
        .send()?;

    if res.status().is_success() {
        Ok(())
    } else {
        Err(res.json::<NewTicketResponse>()?.message.into())
    }
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

/// Data for sending a request to the API for creating a new ticket.
///
/// This are the required fields. There are also optional fields which we currently don't use.
#[derive(Serialize)]
#[serde(rename_all(serialize = "PascalCase"))]
struct NewTicketRequest {
    title: String,
    description: String,
    customer_id: usize,
    address_id: usize,
}

#[derive(Debug, Deserialize)]
struct NewTicketResponse {
    _success: bool,
    _id: usize,
    message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: How can I inspect the client/request config?
    #[test]
    fn client_config() -> Result<(), Box<dyn Error>> {
        let client = build_client("user1", "pw123")?;
        let request = client.post("https://httpbin.org").build()?;
        dbg!(&request);
        // assert_eq!(1, 4);
        Ok(())
    }
}
