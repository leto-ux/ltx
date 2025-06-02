mod patterns;
mod read_config;

use std::env;

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
                eprintln!("Usage: {} - send <address> <amount>", args[0]);
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

            if let Err(e) = patterns::send_ltc(address, amount).await {
                eprintln!("Error sending LTC: {}", e);
            }
        }
        // temp arguments for now, want them for clarity's sake
        // TODO add exit codes using std::process:exit?
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
