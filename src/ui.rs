use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Margin},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
};

use crate::app::{App, AppMode, CredentialField};

const ACCENT: Color = Color::Cyan;
const HIGHLIGHT: Color = Color::Green;
const MUTED: Color = Color::DarkGray;
const DETAILS: Color = Color::Magenta;
const INFO: Color = Color::Yellow;
const PANEL_BORDER: Color = Color::Rgb(82, 96, 122);
const PANEL_BORDER_ACTIVE: Color = Color::Rgb(110, 144, 198);

pub fn draw(f: &mut Frame, app: &mut App) {
    if app.mode == AppMode::Credentials {
        draw_credentials(f, app);
        return;
    }

    let root = f.area();
    let root = root.inner(Margin::new(1, 1));

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(8),
            Constraint::Length(4),
        ])
        .split(root);

    draw_header(f, layout[0], app);

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(28),
            Constraint::Min(30),
            Constraint::Length(34),
        ])
        .split(layout[1]);

    draw_summary(f, body[0], app);
    draw_repo_list(f, body[1], app);
    draw_repo_details(f, body[2], app);

    draw_footer(f, layout[2], app);
}

fn draw_credentials(f: &mut Frame, app: &App) {
    let root = f.area();
    let center = centered_rect(62, 46, root);

    let mut lines = vec![
        Line::from(vec![
            Span::styled(
                "GitHub Repo Control",
                Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from("Enter your GitHub credentials to load repositories."),
        Line::from(""),
        credential_line("Username", app.credential_value(CredentialField::Username), app.active_field == CredentialField::Username),
        credential_line("Token", &app.masked_token(), app.active_field == CredentialField::Token),
        Line::from(""),
    ];

    if let Some(status) = app.status_message.as_deref() {
        lines.push(Line::from(vec![
            Span::styled("status", Style::default().fg(MUTED)),
            Span::raw(format!(": {}", status)),
        ]));
        lines.push(Line::from(""));
    }

    lines.push(Line::from(vec![
        Span::styled("Enter", Style::default().fg(HIGHLIGHT).add_modifier(Modifier::BOLD)),
        Span::raw(" next field / submit   "),
        Span::styled("Tab", Style::default().fg(INFO).add_modifier(Modifier::BOLD)),
        Span::raw(" switch field   "),
        Span::styled("Backspace", Style::default().fg(DETAILS).add_modifier(Modifier::BOLD)),
        Span::raw(" delete char   "),
        Span::styled("q", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        Span::raw(" quit"),
    ]));

    let panel = Paragraph::new(lines)
        .alignment(Alignment::Left)
        .block(panel_block("login", PANEL_BORDER_ACTIVE));

    f.render_widget(panel, center);
}

fn draw_header(f: &mut Frame, area: ratatui::prelude::Rect, app: &App) {
    let welcome = app
        .status_message
        .as_deref()
        .unwrap_or("Welcome. Use the keyboard to navigate and manage repositories.");

    let header = Paragraph::new(vec![
        Line::from(vec![
            Span::styled(
                "GitHub Repo Control",
                Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
            ),
            Span::raw("  •  dashboard view"),
        ]),
        Line::from(vec![
            Span::styled(
                format!("{} repos", app.total_count()),
                Style::default().fg(INFO).add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled(
                format!("{} selected", app.selected_count()),
                Style::default().fg(HIGHLIGHT).add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled(
                format!("cursor {}", app.cursor.saturating_add(1)),
                Style::default().fg(DETAILS),
            ),
        ]),
        Line::from(vec![
            Span::styled("status", Style::default().fg(MUTED)),
            Span::raw(format!(": {}", welcome)),
        ]),
    ])
    .alignment(Alignment::Left)
    .block(panel_block("status", ACCENT));

    f.render_widget(header, area);
}

fn draw_summary(f: &mut Frame, area: ratatui::prelude::Rect, app: &App) {
    let selected = app.selected_count();
    let active = app.current_repo().map(|repo| repo.name.as_str()).unwrap_or("none");
    let has_selection = if app.has_selection() { "yes" } else { "no" };

    let summary = Paragraph::new(vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("total", Style::default().fg(MUTED)),
            Span::raw(format!(": {}", app.total_count())),
        ]),
        Line::from(vec![
            Span::styled("selected", Style::default().fg(MUTED)),
            Span::raw(format!(": {}", selected)),
        ]),
        Line::from(vec![
            Span::styled("has selection", Style::default().fg(MUTED)),
            Span::raw(format!(": {}", has_selection)),
        ]),
        Line::from(vec![
            Span::styled("active repo", Style::default().fg(MUTED)),
            Span::raw(format!(": {}", active)),
        ]),
    ])
    .block(panel_block("overview", PANEL_BORDER_ACTIVE));

    f.render_widget(summary, area);
}

fn draw_repo_list(f: &mut Frame, area: ratatui::prelude::Rect, app: &App) {
    let items: Vec<ListItem> = app
        .repos
        .iter()
        .map(|repo| {
            let mark = if repo.selected { "[x]" } else { "[ ]" };
            ListItem::new(Line::from(vec![
                Span::styled(mark, Style::default().fg(HIGHLIGHT)),
                Span::raw(" "),
                Span::raw(repo.name.as_str()),
            ]))
        })
        .collect();

    let mut state = ListState::default();
    state.select(Some(app.cursor));

    let list = List::new(items)
        .block(panel_block("repositories", ACCENT))
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(ACCENT)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    f.render_stateful_widget(list, area, &mut state);
}

fn draw_repo_details(f: &mut Frame, area: ratatui::prelude::Rect, app: &App) {
    let content = if let Some(repo) = app.current_repo() {
        vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("name", Style::default().fg(MUTED)),
                Span::raw(format!(": {}", repo.name)),
            ]),
            Line::from(vec![
                Span::styled("selected", Style::default().fg(MUTED)),
                Span::raw(format!(": {}", if repo.selected { "yes" } else { "no" })),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled("keys", Style::default().fg(MUTED).add_modifier(Modifier::BOLD))]),
            Line::from("  up/down  move cursor"),
            Line::from("  space    toggle repo"),
            Line::from("  a        select all"),
            Line::from("  u        clear selection"),
            Line::from("  p        make selected private"),
            Line::from("  q        quit"),
        ]
    } else {
        vec![
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "Loading repositories...",
                    Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(""),
            Line::from("The dashboard will populate as soon as GitHub data arrives."),
        ]
    };

    let details = Paragraph::new(content).block(
        panel_block("details", DETAILS),
    );

    f.render_widget(details, area);
}

fn draw_footer(f: &mut Frame, area: ratatui::prelude::Rect, app: &App) {
    let mut pieces = vec![
        Span::styled("q", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        Span::raw(" quit   "),
        Span::styled("space", Style::default().fg(INFO).add_modifier(Modifier::BOLD)),
        Span::raw(" toggle   "),
        Span::styled("a", Style::default().fg(HIGHLIGHT).add_modifier(Modifier::BOLD)),
        Span::raw(" all   "),
        Span::styled("u", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
        Span::raw(" clear   "),
        Span::styled("p", Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)),
        Span::raw(" private"),
    ];

    let extra = app
        .status_message
        .as_deref()
        .filter(|message| !message.is_empty())
        .map(|message| format!("   • {}", message))
        .or_else(|| app.has_selection().then(|| "   • ready to apply private action".to_string()));

    if let Some(message) = extra {
        pieces.push(Span::raw(message));
    }

    let footer = Paragraph::new(Line::from(pieces)).block(
        panel_block("commands", PANEL_BORDER),
    );

    f.render_widget(footer, area);
}

fn credential_line(label: &str, value: &str, active: bool) -> Line<'static> {
    let label_style = Style::default().fg(MUTED);
    let value_style = if active {
        Style::default()
            .fg(Color::Black)
            .bg(ACCENT)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let prefix = if active { "▶ " } else { "  " };

    Line::from(vec![
        Span::raw(prefix),
        Span::styled(format!("{}", label), label_style),
        Span::raw(": "),
        Span::styled(value.to_string(), value_style),
    ])
}

fn centered_rect(percent_x: u16, percent_y: u16, area: ratatui::prelude::Rect) -> ratatui::prelude::Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1])[1]
}

fn panel_block(title: &'static str, border: Color) -> Block<'static> {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(border))
}
