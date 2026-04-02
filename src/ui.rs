use std::{os::linux::raw::stat, vec};

use ratatui::{
    Frame,
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crate::app::App;

pub fn draw(f: &mut Frame, app: &mut App) {
    let items: Vec<ListItem> = app
        .repos
        .iter()
        .map(|repo| {
            let mark = if repo.selected { "✔" } else { " " };
            ListItem::new(format!("{} {}", mark, repo.name))
        })
        .collect();

    let mut state = ListState::default();
    state.select(Some(app.cursor));

    let list = List::new(items)
        .block(
            Block::default()
                .title("Github repo control(space=select, a=all, p=private, q=quit)")
                .borders(Borders::ALL),
        )
        .highlight_symbol(">>");
    let area = f.size();
    f.render_stateful_widget(list, area, &mut state);
}
