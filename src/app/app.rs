use crate::connection::connection::{Connection, ConnectionList};
use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{
        palette::tailwind::{BLUE, SLATE},
        Color, Modifier, Style, Stylize,
    },
    symbols,
    text::Line,
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, Padding, Paragraph, StatefulWidget,
        Widget, Wrap,
    },
    DefaultTerminal,
};

const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = SLATE.c200;

pub struct App {
    should_exit: bool,
    connections: ConnectionList,
}

impl Default for App {
    fn default() -> Self {
        Self {
            should_exit: false,
            connections: ConnectionList::new(),
        }
    }
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_exit {
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
        }
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        let [list_area, item_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(main_area);

        App::render_header(header_area, buf);
        App::render_footer(footer_area, buf);
        self.render_list(list_area, buf);
        self.render_selected_item(item_area, buf);
    }
}

/// Rendering logic for the app
impl App {
    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("OpenVPN Connection Manager")
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use ↓↑ to move, ENTER to select, q to quit")
            .centered()
            .render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("VPN List").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG);

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
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.connections.state);
    }

    fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
        let info = if let Some(i) = self.connections.state.selected() {
            let connection = &self.connections.items[i];
            format!("Path: {}", connection.path)
        } else {
            "No item selected".to_string()
        };

        let block = Block::new()
            .title(Line::raw("Path").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG)
            .padding(Padding::horizontal(1));

        Paragraph::new(info)
            .block(block)
            .fg(TEXT_FG_COLOR)
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }
}
