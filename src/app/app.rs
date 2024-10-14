use crate::connection::{
    connection::{Connection, ConnectionList},
    openvpn::OpenVpnConnection,
};
use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{palette::tailwind::SLATE, Color, Modifier, Style, Stylize},
    symbols,
    text::Line,
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, Padding, Paragraph, StatefulWidget,
        Widget, Wrap,
    },
    DefaultTerminal,
};

const NORMAL_ROW_BG: Color = Color::Reset;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = SLATE.c200;

pub struct App {
    should_exit: bool,
    connections: ConnectionList,
    open_vpn_connection: Option<OpenVpnConnection>,
    output: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            should_exit: false,
            connections: ConnectionList::new(),
            open_vpn_connection: None,
            output: String::new(),
        }
    }
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            if self.open_vpn_connection.is_some() {
                if let Some(openvpn_connection) = self.open_vpn_connection.as_mut() {
                    let _ = openvpn_connection.stop()?;
                }
            }
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') => self.should_exit = true,
            KeyCode::Esc => self.select_none(),
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
            KeyCode::Char('g') | KeyCode::Home => self.select_first(),
            KeyCode::Char('G') | KeyCode::End => self.select_last(),
            KeyCode::Enter => {
                self.select_item();
            }
            _ => {}
        }
    }

    fn select_none(&mut self) {
        self.connections.state.select(None);
    }

    fn select_next(&mut self) {
        self.connections.state.select_next();
    }
    fn select_previous(&mut self) {
        self.connections.state.select_previous();
    }

    fn select_first(&mut self) {
        self.connections.state.select_first();
    }

    fn select_last(&mut self) {
        self.connections.state.select_last();
    }

    fn select_item(&mut self) {
        for i in 0..self.connections.items.len() {
            self.connections.items[i].selected = false;
        }

        if let Some(i) = self.connections.state.selected() {
            self.connections.items[i].selected = true;
            let connection = &self.connections.items[i];
            let mut openvpn_connection = OpenVpnConnection::new(connection.clone());

            match openvpn_connection.connect() {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Could not connect to the VPN: {}", e);
                }
            }

            self.open_vpn_connection = Some(openvpn_connection);
        }
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        App::render_header(header_area, buf);
        App::render_footer(footer_area, buf);
        self.render_main_area(main_area, buf);
    }
}

/// Rendering logic for the app
impl App {
    fn render_header(area: Rect, buf: &mut Buffer) {
        let header_text: &str = "OpenVPN Connection Manager";
        Paragraph::new(header_text)
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        let footer_text: &str = "Use ↓↑ to move, ENTER to select, q to quit";
        Paragraph::new(footer_text).centered().render(area, buf);
    }

    fn render_main_area(&mut self, area: Rect, buf: &mut Buffer) {
        let outer_block = Block::bordered().border_set(symbols::border::EMPTY);
        let inner_area = outer_block.inner(area);

        let [list_area, output_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Percentage(70)]).areas(inner_area);

        outer_block.render(area, buf);
        self.render_list_block(list_area, buf);
        self.render_connection_output(output_area, buf);
    }

    fn render_list_block(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw(" VPN List "))
            .bold()
            .borders(Borders::ALL)
            .border_set(symbols::border::ROUNDED);

        let items: Vec<ListItem> = self
            .connections
            .items
            .iter()
            .enumerate()
            .map(|(_, connection)| {
                let color: Color = if connection.selected {
                    SLATE.c800
                } else {
                    NORMAL_ROW_BG
                };
                Connection::to_list_item(connection).bg(color)
            })
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol("> ")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.connections.state);
    }

    fn render_connection_output(&mut self, area: Rect, buf: &mut Buffer) {
        let info = if let Some(i) = self.connections.state.selected() {
            if self.open_vpn_connection.is_none() {
                let connection = &self.connections.items[i];
                format!("Path: {}", connection.path)
            } else {
                let openvpn_connection = self.open_vpn_connection.as_ref().unwrap();
                let stdout_buffer = openvpn_connection.stdout_buffer.lock().unwrap();
                let stderr_buffer = openvpn_connection.stderr_buffer.lock().unwrap();

                if stderr_buffer.is_empty() {
                    format!("{}\n", stdout_buffer.trim())
                } else {
                    format!("{}\n\nERR: {}", stdout_buffer.trim(), stderr_buffer.trim())
                }
            }
        } else {
            "No output".to_string()
        };

        self.output = info.clone();

        let block = Block::new()
            .title(Line::raw(" Output "))
            .bold()
            .borders(Borders::ALL)
            .border_set(symbols::border::ROUNDED)
            .padding(Padding::horizontal(1));

        Paragraph::new(self.output.clone())
            .block(block)
            .fg(TEXT_FG_COLOR)
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }
}
