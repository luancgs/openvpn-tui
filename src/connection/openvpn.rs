use std::{
    io::{self, BufRead, BufReader},
    process::{Command, Stdio},
    sync::{Arc, Mutex},
    thread::{self},
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

        let stdout_thread = thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                let mut line = line.unwrap();
                line = format!("{}\n", line);
                let mut stdout_buffer = stdout_buffer.lock().unwrap();
                stdout_buffer.push_str(&line);
            }
        });

        let stderr_thread = thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                let mut line = line.unwrap();
                line = format!("{}\n", line);
                let mut stderr_buffer = stderr_buffer.lock().unwrap();
                stderr_buffer.push_str(&line);
            }
        });

        stdout_thread.join().unwrap();
        stderr_thread.join().unwrap();

        let status = child.wait()?;
        if !status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("OpenVPN exited with status: {}", status),
            ));
        }

        Ok(())
    }
}
