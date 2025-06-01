use reqwest::Client;
use serde_json::json;
use std::env;

async fn send_ltc(address: &str, amount: f64) -> Result<(), Box<dyn std::error:Error>> {
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
    println!("Response: {}", text);

    Ok(())
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <options> [args...]", args[0]);
        return;
    }

    match args[1].as_str() {
        "-send" => {
            if args.len() != 4 {
                eprintln("Usage: {} - send <address> <amount>", args[0]);
                return;
            }

            let address = &args[2];
            let amount: f64 = match args[3].parse() {
                Ok(val) => val,
                Err(_) => {
                    eprintln!("Invalid amount: {}", args[3]);
                    return;
                }
            };

            if let Err(e) = send_ltc(address, amount).await {
                eprintln!("Error sending LTC: {}", e);
            }
        }
        // temp arguments for now, want them for clarity's sake
        "-balance" => {
                    println!("(Placeholder) Fetching wallet balance...");
        }

        "-list" => {
            println!("(Placeholder) Listing recent transactions...");
        }

        "-help" | "--help" => {
            println!("Usage:");
            println!("  {} -send <address> <amount>", args[0]);
            println!("  {} -balance", args[0]);
            println!("  {} -list", args[0]);
            println!("  {} -help", args[0]);
        }

        _ => {
            eprintln!("Unknown option: {}", args[1]);
            eprintln!("Use -help to see available commands.");
        }
    }
}
