mod api;
mod app;
mod config;
mod event;
mod ui;

use anyhow::Result;
use app::App;
use config::Config;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use event::{handle_key_event, poll_events, AppEvent, EventHandler};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::from_env()?;

    let client = api::ApiClient::new(config.donetick_url, config.donetick_token)?;

    let mut terminal = setup_terminal()?;

    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic| {
        restore_terminal().ok();
        original_hook(panic);
    }));

    let result = run_app(&mut terminal, client).await;

    restore_terminal()?;

    result
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    client: api::ApiClient,
) -> Result<()> {
    let mut app = App::new();
    let mut event_handler = EventHandler::new();
    let tx = event_handler.sender();

    let client_clone = client.clone();
    let tx_clone = tx.clone();
    tokio::spawn(async move {
        match client_clone.list_chores().await {
            Ok(tasks) => {
                let _ = tx_clone.send(AppEvent::TasksLoaded(tasks));
            }
            Err(e) => {
                let _ = tx_clone.send(AppEvent::Error(e.to_string()));
            }
        }
    });

    let tx_clone = tx.clone();
    tokio::spawn(async move {
        poll_events(tx_clone).await;
    });

    while app.running {
        terminal.draw(|f| ui::draw(f, &app))?;

        if let Some(event) = event_handler.next().await {
            match event {
                AppEvent::Input(key) => {
                    handle_key_event(key, &mut app, &client, &event_handler.sender()).await;
                }
                AppEvent::TasksLoaded(tasks) => {
                    app.tasks = tasks;
                    app.loading = false;
                    if !app.tasks.is_empty() && app.selected_task >= app.tasks.len() {
                        app.selected_task = 0;
                    }
                }
                AppEvent::TaskAdded => {
                    app.show_task_list();
                    let client = client.clone();
                    let tx = event_handler.sender();
                    tokio::spawn(async move {
                        match client.list_chores().await {
                            Ok(tasks) => {
                                let _ = tx.send(AppEvent::TasksLoaded(tasks));
                            }
                            Err(e) => {
                                let _ = tx.send(AppEvent::Error(e.to_string()));
                            }
                        }
                    });
                }
                AppEvent::TaskCompleted => {
                    let client = client.clone();
                    let tx = event_handler.sender();
                    tokio::spawn(async move {
                        match client.list_chores().await {
                            Ok(tasks) => {
                                let _ = tx.send(AppEvent::TasksLoaded(tasks));
                            }
                            Err(e) => {
                                let _ = tx.send(AppEvent::Error(e.to_string()));
                            }
                        }
                    });
                }
                AppEvent::Error(msg) => {
                    app.show_error(msg);
                }
            }
        }
    }

    Ok(())
}
