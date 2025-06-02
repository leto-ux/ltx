use dirs::{self, home_dir};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug, Default)]
pub struct LTCConfig {
    rpc_user: Option<String>,
    rpc_password: Option<String>,
}

fn read_rpc_credentials(file_path: &str) -> io::Result<LTCConfig> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut config = LTCConfig::default();

    for line_buffer in reader.lines() {
        let line = line_buffer?; // made this way so that i can see the errors better, not sure if
                                 // thats the cleanest way to do it
        let line_trim = line.trim();

        if line_trim.starts_with("rpcuser=") {
            if let Some((_, value)) = line_trim.split_once("=") {
                config.rpc_user = Some(value.trim().to_string());
            }
        } else if line_trim.starts_with("rpcpassword") {
            if let Some((_, value)) = line_trim.split_once("=") {
                config.rpc_password = Some(value.trim().to_string());
            }
        }
    }

    Ok(config)
}

// I have no bloody clue as to how to return a struct here
pub fn verify_config() -> [String; 2] {
    let config_file_path_buf = if let Some(mut home_dir) = dirs::home_dir() {
        home_dir.push(".litecoin");
        home_dir.push("litecoin.conf");
        home_dir
    } else {
        panic!("Error reading home dir, falling back to ./litecoin.conf");
    };

    let config_file_path = config_file_path_buf.to_str();

    println!("reading from '{:?}'", config_file_path);

    match read_rpc_credentials(config_file_path.expect("default")) {
        Ok(config) => {
            println!("\nSuccessfully read RPC configuration:");
            if let Some(ref user) = config.rpc_user {
                println!("  RPC User: '{}'", user);
            } else {
                println!("  RPC User: Not found or empty.");
            }
            if let Some(ref password) = config.rpc_password {
                println!("  RPC Password: '{}'", password); // TODO remove printing
            } else {
                println!("  RPC Password: Not found or empty.");
            }

            [
                config.rpc_user.expect("error checking done ahead of time"),
                config.rpc_password.expect("same story"),
            ]
        }
        Err(e) => {
            eprintln!("Error reading config file '{:?}': {}", config_file_path, e);
            [Default::default(), Default::default()]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::verify_config;

    #[test]
    fn verify_config_test() {
        assert_eq!(verify_config(), ["testnet01", "testnet01"]);
    }
}
