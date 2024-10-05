use ::std::io;

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    style::Stylize,
    widgets::Paragraph,
    DefaultTerminal,
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = run(terminal);
    ratatui::restore();
    app_result
}

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let bg_color: ratatui::style::Color = "#0000ff".parse().unwrap();
            let fg_color: ratatui::style::Color = "#ffffff".parse().unwrap();
            let greeting = Paragraph::new("Hello, Ratatui! (press 'q' to quit)")
                .bg(bg_color)
                .fg(fg_color);
            frame.render_widget(greeting, frame.area());
        })?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}
