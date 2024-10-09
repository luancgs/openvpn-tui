use std::{
    io::{self, Read},
    process::{Command, Stdio},
    sync::{Arc, Mutex},
};

use super::connection::Connection;

pub struct OpenVpnConnection {
    pub connected: bool,
    connection: Connection,
    pub stdout_buffer: Arc<Mutex<String>>,
    pub stderr_buffer: Arc<Mutex<String>>,
}

impl OpenVpnConnection {
    pub fn new(connection: Connection) -> Self {
        Self {
            connected: false,
            connection,
            stdout_buffer: Arc::new(Mutex::new(String::new())),
            stderr_buffer: Arc::new(Mutex::new(String::new())),
        }
    }

    pub fn connect(&mut self) -> io::Result<()> {
        if self.connected {
            return Ok(());
        }

        let openvpn_check = Command::new("openvpn")
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();

        match openvpn_check {
            Ok(status) if status.success() => {}
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "OpenVPN is not installed",
                ));
            }
        }

        let mut child = Command::new("sh")
            .arg("-c")
            .arg(format!(
                "$(which openvpn) --config {}",
                self.connection.path
            ))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdout = child.stdout.take().expect("Failed to take stdout");
        let stderr = child.stderr.take().expect("Failed to take stderr");

        let stdout_buffer = Arc::clone(&self.stdout_buffer);
        let stderr_buffer = Arc::clone(&self.stderr_buffer);

        let stdout_thread = std::thread::spawn(move || {
            let mut reader = io::BufReader::new(stdout);
            let mut buffer = String::new();
            while let Ok(bytes_read) = reader.read_to_string(&mut buffer) {
                if bytes_read == 0 {
                    break;
                }
                let mut stdout_buffer = stdout_buffer.lock().unwrap();
                stdout_buffer.push_str(&buffer);
                print!("{}", buffer);
                buffer.clear();
            }
        });

        // Handle stderr in a separate thread
        let stderr_thread = std::thread::spawn(move || {
            let mut reader = io::BufReader::new(stderr);
            let mut buffer = String::new();
            while let Ok(bytes_read) = reader.read_to_string(&mut buffer) {
                if bytes_read == 0 {
                    break;
                }
                let mut stderr_buffer = stderr_buffer.lock().unwrap();
                stderr_buffer.push_str(&buffer);
                eprint!("{}", buffer);
                buffer.clear();
            }
        });

        // Wait for the threads to finish
        stdout_thread.join().unwrap();
        stderr_thread.join().unwrap();

        // Wait for the child process to exit
        let status = child.wait()?;
        if !status.success() {
            eprintln!("OpenVPN process exited with status: {}", status);
        }

        Ok(())
    }
}
