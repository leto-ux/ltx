use crate::read_config::LTCConfig;

use reqwest::{Client, Response};
use serde_json::{json, Map, Value};

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
        .post("http://host.docker.internal:19332/wallet/maninthemiddle")
        // .post("http://172.17.0.1:19332/wallet/silksong")
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
    let parsed: serde_json::Value = serde_json::from_str(&to_text)?;
    println!("1");

    match parsed["error"].as_str() {
        None => match parsed["result"].as_str() {
            Some(address) => {
                println!("2");
                println!("{}", address);
            }
            None => {
                println!("3");
                panic!("\"result\" field is missing or is not a string!");
            }
        },
        Some(err) => {
            println!("4");
            println!("{}", err);
        }
    }

    println!("5");
    Ok(())
}

pub async fn get_new_address(
    config: &LTCConfig,
    label: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let (username, password) = set_username_password(config);

    let body = json!({
        "jsonrpc": "1.0",
        "id": "curltest",
        "method": "getnewaddress",
        "params": [label]
    });

    let response = set_response(&client, &username, &password, &body).await?;
    let to_text = response.text().await?;
    let parsed: serde_json::Value = serde_json::from_str(&to_text)?;

    match parsed["error"].as_str() {
        None => match parsed["result"].as_str() {
            Some(address) => println!("{}", address),
            None => panic!("\"result\" field is missing or is not a string!"),
        },
        Some(err) => println!("{}", err),
    }

    Ok(())
}

pub async fn list_transactions(config: &LTCConfig) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let (username, password) = set_username_password(config);
    const TRANSACTION_HISTORY: usize = 100;

    let body = json!({
        "jsonrpc": "1.0",
        "id": "curltest",
        "method": "listtransactions",
        // probably a reliability risk, doesn't matter for a project, but would were someone to
        // say, screw it, if Ross got pardoned, then so could I
        "params": ["*", TRANSACTION_HISTORY, 0],
    });

    let response = set_response(&client, &username, &password, &body).await?;
    let to_text = response.text().await?;
    let parsed: serde_json::Value = serde_json::from_str(&to_text)?;

    let mut transactions = Vec::new();

    for i in 0..TRANSACTION_HISTORY {
        match parsed["result"][i]["address"].as_str() {
            Some(address) => {
                let amount = parsed["result"][i]["amount"].clone();
                transactions.push(json!({
                    "address": address,
                    "amount": amount
                }));
            }
            None => break,
        };
    }
    println!("{}", serde_json::to_string(&transactions).unwrap());

    Ok(())
}

// #[allow(dead_code, unused_variables)] // temp so that bacon chills out a bit
pub async fn list_address_groupings(config: &LTCConfig) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let (username, password) = set_username_password(config);

    let body = json!({
        "jsonrpc": "1.0",
        "id": "curltest",
        "method": "listaddressgroupings",
        "params": []
    });

    let response = set_response(&client, &username, &password, &body).await?;

    let to_text = response.text().await?;
    println!("{}", to_text);

    Ok(())
}

pub async fn get_balance(
    config: &LTCConfig,
    confirmation_count: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let (username, password) = set_username_password(config);

    let body = json!({
        "jsonrpc": "1.0",
        "id": "curltest",
        "method": "getbalance",
        "params": ["*", confirmation_count]
    });

    let response = set_response(&client, &username, &password, &body).await?;

    let to_text = response.text().await?;
    println!("{}", to_text);

    Ok(())
}
