use ratatui::widgets::{ListItem, ListState};

use super::files::list_vpn_files;

#[derive(Debug)]
pub struct Connection {
    pub title: String,
    pub path: String,
    pub selected: bool,
}

impl Connection {
    pub fn new(selected: bool, title: &str, path: &str) -> Self {
        Self {
            selected,
            title: title.to_string(),
            path: path.to_string(),
        }
    }

    pub fn to_list_item(&self) -> ListItem {
        ListItem::new(self.title.clone())
    }
}

pub struct ConnectionList {
    pub items: Vec<Connection>,
    pub state: ListState,
}

impl ConnectionList {
    pub fn new() -> Self {
        let items = list_vpn_files()
            .into_iter()
            .map(|vpn_file| Connection::new(false, &vpn_file.title, &vpn_file.path))
            .collect();
        let state = ListState::default();
        Self { items, state }
    }
}

impl FromIterator<(bool, &'static str, &'static str)> for ConnectionList {
    fn from_iter<I: IntoIterator<Item = (bool, &'static str, &'static str)>>(iter: I) -> Self {
        let items = iter
            .into_iter()
            .map(|(selected, title, path)| Connection::new(selected, title, path))
            .collect();
        let state = ListState::default();
        Self { items, state }
    }
}
