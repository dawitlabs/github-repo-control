mod app;
mod events;
mod github;
mod models;
mod ui;

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::error::Error;
use std::io;

use app::App;
use github::{fetch_repos, make_private};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Your Github username: ");
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    let username = username.trim().to_string();

    println!("Your Github token: ");
    let mut token = String::new();
    io::stdin().read_line(&mut token)?;
    let token = token.trim().to_string();

    print!("Fetching repos...");
    let repos = fetch_repos(&username, &token).await?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(repos);

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        if let Some(event) = events::read_event()? {
            match event {
                events::AppEvent::Quit => break,
                events::AppEvent::Down => app.next(),
                events::AppEvent::Up => app.previous(),
                events::AppEvent::Toggle => app.toggle(),
                events::AppEvent::SelectAll => app.select_all(),
                events::AppEvent::MakePrivate => {
                    let selected = app.selected_repos();

                    for repo in selected {
                        make_private(&username, &token, &repo).await?;
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
