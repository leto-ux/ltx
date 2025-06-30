use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug, Default)]
pub struct LTCConfig {
    pub rpc_user: Option<String>,
    pub rpc_password: Option<String>,
}

fn read_credentials(file_path: &str) -> io::Result<LTCConfig> {
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

pub fn read_credentials_verified() -> io::Result<LTCConfig> {
    let config_file_path = dirs::home_dir()
        .unwrap_or_else(|| panic!("Cannot determine home dir"))
        .join(".litecoin")
        .join("litecoin.conf");

    // println!("reading from '{:?}'", config_file_path);

    let config = read_credentials(
        config_file_path
            .to_str()
            .expect("Failed to convert path to string"),
    )?;

    // have to use &String as String doesn't have the Copy trait
    match (&config.rpc_user, &config.rpc_password) {
        (Some(_), Some(_)) => Ok(config),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "username or password missing in config",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::read_credentials_verified;

    #[test]
    fn read_credentials_verified_test() {
        let config = read_credentials_verified().expect("Failed to read config");

        // as_deref converts my Option<String> to Option<&str>
        // Some converts my &str to a Option<&str>
        // this language is so weird man i love it
        assert_eq!(config.rpc_user.as_deref(), Some("testnet01"));
        assert_eq!(config.rpc_password.as_deref(), Some("testnet01"));
    }
}
