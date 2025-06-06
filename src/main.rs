mod patterns;
mod read_config;
use crate::read_config::read_credentials_verified;

use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let config = read_credentials_verified().expect("Failed to load config");

    if args.len() < 2 {
        eprintln!("Usage: {} <options> [args...]", args[0]);
        return;
    }

    match args[1].as_str() {
        "--sendtoaddress" => {
            if args.len() != 4 {
                eprintln!("Usage: {} -sendtoaddress <address> <amount>", args[0]);
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

            if let Err(e) = patterns::send_to_address(&config, address, amount).await {
                eprintln!("Error sending LTC: {}", e);
            }
        }

        // temp arguments for now, want them for clarity's sake
        // TODO add exit codes using std::process:exit?
        "--getnewaddress" => match args.len() {
            2 => {
                if let Err(e) = patterns::get_new_address(&config, "").await {
                    eprintln!("Error generating a new address: {}", e);
                }
            }

            3 => {
                let label = &args[2];

                if let Err(e) = patterns::get_new_address(&config, label).await {
                    eprintln!("Error generating a new address: {}", e);
                }
            }

            _ => {
                eprintln!("Usage: {} -getnewaddress <label (optional)>", args[0]);
            }
        },

        "--getbalance" => match args.len() {
            2 => {
                if let Err(e) = patterns::get_balance(&config, 0).await {
                    eprintln!("Error generating a new address: {}", e);
                }
            }

            3 => {
                let confirmation_count: u32 = match args[2].parse() {
                    Ok(val) => val,
                    Err(_) => {
                        eprintln!("Invalid confirmation count: {}", args[2]);
                        return;
                    }
                };

                if let Err(e) = patterns::get_balance(&config, confirmation_count).await {
                    eprintln!("Error getting balance: {}", e);
                }
            }

            _ => {
                eprintln!(
                    "Usage: {} -getbalance <confirmation_count (optional)>",
                    args[0]
                );
            }
        },

        "--listaddressgroupings" => {
            println!("(Placeholder) Fetching wallet balance...");
        }

        "--list" => {
            println!("(Placeholder) Listing recent transactions...");
        }

        "--help" | "-h" => {
            println!("Usage:");
            println!("  {} --sendtoaddress <address> <amount>", args[0]);
            println!("  {} --getnewaddress <label (optional)>", args[0]);
            println!("  {} --getbalance <confirmation_count(optional)>", args[0]);
            println!("  {} --listaddressgroupings", args[0]);
            println!("  {} --help", args[0]);
        }

        _ => {
            eprintln!("Unknown option: {}", args[1]);
            eprintln!("Use --help to see available commands.");
        }
    }
}
