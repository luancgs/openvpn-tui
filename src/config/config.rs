use serde::Deserialize;
use std::{fs, process};

use dirs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub vpn_files_path: String,
}

impl Config {
    fn get_config_path() -> String {
        let config_dir = dirs::config_dir();

        match config_dir {
            Some(path) => path.to_str().unwrap().to_string(),
            None => {
                eprintln!("Could not find the config directory");
                process::exit(1);
            }
        }
    }

    pub fn default() -> Self {
        let home_dir = dirs::home_dir();

        let config = match home_dir {
            Some(path) => match path.to_str() {
                Some(path) => {
                    let vpn_files_path = format!("{}/.vpns", path);
                    Config { vpn_files_path }
                }
                None => {
                    eprintln!("Could not convert the home directory to a string");
                    process::exit(1);
                }
            },
            None => {
                eprintln!("Could not find the home directory");
                process::exit(1);
            }
        };

        config
    }

    pub fn from_file() -> Self {
        let config_path = Config::get_config_path();
        let config_file = format!("{}/openvpn-tui.toml", config_path);

        let config = match fs::read_to_string(config_file) {
            Ok(contents) => match toml::from_str(&contents) {
                Ok(config) => config,
                Err(_) => Config::default(),
            },
            Err(_) => Config::default(),
        };

        config
    }
}
