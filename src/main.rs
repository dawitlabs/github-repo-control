mod app;
mod events;
mod github;
mod models;
mod ui;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::error::Error;
use std::io;

use app::{App, AppMode, CredentialField};
use github::{fetch_repos, make_private};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result: Result<(), Box<dyn Error>> = async {
        let mut app = App::new(Vec::new());
        app.set_status("Enter your GitHub username and token.");

        terminal.draw(|f| ui::draw(f, &mut app))?;

        loop {
            terminal.draw(|f| ui::draw(f, &mut app))?;

            let event = event::read()?;

            match app.mode {
                AppMode::Credentials => {
                    if let Event::Key(key) = event {
                        match key.code {
                            KeyCode::Char('q') => break,
                            KeyCode::Tab => {
                                app.next_credential_field();
                                app.clear_status();
                            }
                            KeyCode::Enter => {
                                if app.active_field == CredentialField::Username {
                                    app.next_credential_field();
                                    app.clear_status();
                                } else if app.credentials_ready() {
                                    app.mode = AppMode::Loading;
                                    app.set_status("Fetching repositories...");
                                    terminal.draw(|f| ui::draw(f, &mut app))?;

                                    let repos = fetch_repos(app.username(), app.token()).await;
                                    match repos {
                                        Ok(repos) => {
                                            app.repos = repos;
                                            app.cursor = 0;
                                            app.mode = AppMode::Dashboard;
                                            app.set_status("Welcome. Pick repos, then press p to make them private.");
                                        }
                                        Err(error) => {
                                            app.repos.clear();
                                            app.mode = AppMode::Credentials;
                                            app.active_field = CredentialField::Username;
                                            app.set_status(format!("Failed to fetch repositories: {}", error));
                                        }
                                    }
                                } else {
                                    app.set_status("Username and token are required.");
                                }
                            }
                            KeyCode::Backspace | KeyCode::Delete => {
                                app.pop_credential_char();
                            }
                            KeyCode::Char(character) if !character.is_control() => {
                                app.push_credential_char(character);
                            }
                            _ => {}
                        }
                    }
                }
                AppMode::Dashboard => {
                    if let Event::Key(key) = event {
                        if let Some(app_event) = events::app_event_from_key_code(key.code) {
                            match app_event {
                                events::AppEvent::Quit => break,
                                events::AppEvent::Down => app.next(),
                                events::AppEvent::Up => app.previous(),
                                events::AppEvent::Toggle => app.toggle(),
                                events::AppEvent::SelectAll => app.select_all(),
                                events::AppEvent::UnselectAll => app.unselect_all(),
                                events::AppEvent::MakePrivate => {
                                    let selected = app.selected_repos();
                                    let total = selected.len();

                                    if total == 0 {
                                        app.set_status("No repositories selected to make private.");
                                        continue;
                                    }

                                    for (index, repo) in selected.into_iter().enumerate() {
                                        app.set_status(format!(
                                            "Privating {}/{}: {}",
                                            index + 1,
                                            total,
                                            repo
                                        ));
                                        terminal.draw(|f| ui::draw(f, &mut app))?;
                                        make_private(app.username(), app.token(), &repo).await?;
                                    }

                                    app.set_status(format!("Finished privating {} repositories.", total));
                                }
                            }
                        }
                    }
                }
                AppMode::Loading => {}
                }
            }

        Ok(())
    }
    .await;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    result
}
