use std::{fs, process};

use crate::config::config;

pub struct VpnFile {
    pub path: String,
    pub title: String,
}

pub fn list_vpn_files() -> Vec<VpnFile> {
    let mut files = vec![];

    let vpn_dir = match fs::read_dir(config::Config::from_file().vpn_files_path) {
        Ok(vpn_dir) => vpn_dir,
        Err(_) => {
            eprintln!("Could not read the VPN files directory");
            process::exit(1);
        }
    };

    for dir_entry in vpn_dir {
        let file = match dir_entry {
            Ok(file) => Some(file),
            Err(_) => {
                eprintln!("Could not read a file in the VPN files directory");
                process::exit(1);
            }
        };

        let path = match file {
            Some(file) => file.path(),
            None => {
                eprintln!("Could not get the path of a file in the VPN files directory");
                process::exit(1);
            }
        };

        let title = match path.file_name() {
            Some(file_name) => match file_name.to_str() {
                Some(file_name) => file_name.to_string(),
                None => {
                    eprintln!("Could not convert a file name to a string");
                    process::exit(1);
                }
            },
            None => {
                eprintln!("Could not get the file name of a file in the VPN files directory");
                process::exit(1);
            }
        };

        let path_string = match path.to_str() {
            Some(path_string) => path_string.to_string(),
            None => {
                eprintln!("Could not convert a path to a string");
                process::exit(1);
            }
        };

        files.push(VpnFile {
            path: path_string,
            title,
        });
    }

    files
}
