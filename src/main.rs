mod app;
mod config;
mod connection;

use std::process::{exit, Command, Stdio};

use app::app::App;
use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    check_dependencies();
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal);
    ratatui::restore();
    app_result
}

fn check_dependencies() {
    let openvpn_check = Command::new("openvpn")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    if !openvpn_check.is_ok_and(|result| result.success()) {
        eprintln!("OpenVPN is not installed or not found in PATH");
        exit(1);
    }
}
