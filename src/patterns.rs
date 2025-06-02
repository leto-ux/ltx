use crate::read_config;

use reqwest::Client;
use serde_json::json;

pub async fn send_ltc(address: &str, amount: f64) -> Result<(), Box<dyn std::error::Error>> {
    let username = "testnet01";
    let password = "testnet01";

    let client = Client::new();

    let json_body = json!({
        "jsonrpc": "1.0",
        "id": "send",
        "method": "sendtoaddress",
        "params": [address, amount]
    });

    let json_response = client
        .post("http://127.0.0.1:19332/")
        .basic_auth(username, Some(password))
        .header("content-type", "text/plain")
        .json(&json_body)
        .send()
        .await?;

    let to_text = json_response.text().await?;
    println!("Response: {}", to_text);

    Ok(())
}
