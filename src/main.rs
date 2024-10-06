mod app;
mod config;
mod connection;

use app::app::App;
use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal);
    ratatui::restore();
    app_result
}
