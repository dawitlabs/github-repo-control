use crossterm::event::{self, Event, KeyCode};
use std::error::Error;

pub enum AppEvent {
    Quit,
    Down,
    Up,
    Toggle,
    SelectAll,
    MakePrivate,
}

pub fn read_event() -> Result<Option<AppEvent>, Box<dyn Error>> {
    if let Event::Key(key) = event::read()? {
        let event = match key.code {
            KeyCode::Char('q') => Some(AppEvent::Quit),
            KeyCode::Down => Some(AppEvent::Down),
            KeyCode::Up => Some(AppEvent::Up),
            KeyCode::Char(' ') => Some(AppEvent::Toggle),
            KeyCode::Char('a') => Some(AppEvent::SelectAll),
            KeyCode::Char('p') => Some(AppEvent::MakePrivate),

            _ => None,
        };
        return Ok(event);
    }
    Ok(None)
}
