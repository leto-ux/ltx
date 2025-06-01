use crate::read_config::LTCConfig;

use reqwest::{Client, Response};
use serde_json::{json, Value};

fn set_username_password(config: &LTCConfig) -> (String, String) {
    let username = config
        .rpc_user
        .as_ref()
        .unwrap_or_else(|| panic!("Couldn't read the password"));

    let password = config
        .rpc_password
        .as_ref()
        .unwrap_or_else(|| panic!("Couldn't read the passord"));

    (username.to_string(), password.to_string())
}

async fn set_response(
    client: &Client,
    username: &String,
    password: &String,
    body: &Value,
) -> Result<Response, reqwest::Error> {
    client
        .post("http://127.0.0.1:19332/")
        .basic_auth(username, Some(password))
        .header("content-type", "text/plain")
        .json(&body)
        .send()
        .await
}

pub async fn send_to_address(
    config: &LTCConfig,
    address: &str,
    amount: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let (username, password) = set_username_password(config);

    let body = json!({
        "jsonrpc": "1.0",
        "id": "send",
        "method": "sendtoaddress",
        "params": [address, amount]
    });

    let response = set_response(&client, &username, &password, &body).await?;

    let to_text = response.text().await?;
    println!("Response: {}", to_text);

    Ok(())
}

#[allow(dead_code, unused_variables)] // temp so that bacon chills out a bit
pub async fn get_new_address(config: &LTCConfig) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let (username, password) = set_username_password(config);

    let body = json!({
        "jsonrpc": "1.0",
        "id": "send",
        "method": "getnewaddress",
        "params": []
    });

    let response = set_response(&client, &username, &password, &body).await?;

    let to_text = response.text().await?;
    println!("Response: {}", to_text);

    Ok(())
}
