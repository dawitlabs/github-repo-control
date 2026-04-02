use crossterm::event::KeyCode;

pub enum AppEvent {
    Quit,
    Down,
    Up,
    Toggle,
    SelectAll,
    UnselectAll,
    MakePrivate,
}

pub fn app_event_from_key_code(key_code: KeyCode) -> Option<AppEvent> {
    match key_code {
        KeyCode::Char('q') => Some(AppEvent::Quit),
        KeyCode::Down => Some(AppEvent::Down),
        KeyCode::Up => Some(AppEvent::Up),
        KeyCode::Char(' ') => Some(AppEvent::Toggle),
        KeyCode::Char('a') => Some(AppEvent::SelectAll),
        KeyCode::Char('u') => Some(AppEvent::UnselectAll),
        KeyCode::Char('p') => Some(AppEvent::MakePrivate),
        _ => None,
    }
}
