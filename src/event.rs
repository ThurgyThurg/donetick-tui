use crate::api::{ApiClient, Chore};
use crate::app::{App, View};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use tokio::sync::mpsc;

#[derive(Debug)]
pub enum AppEvent {
    Input(KeyEvent),
    TasksLoaded(Vec<Chore>),
    TaskAdded,
    TaskCompleted,
    Error(String),
}

pub struct EventHandler {
    tx: mpsc::UnboundedSender<AppEvent>,
    rx: mpsc::UnboundedReceiver<AppEvent>,
}

impl EventHandler {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self { tx, rx }
    }

    pub fn sender(&self) -> mpsc::UnboundedSender<AppEvent> {
        self.tx.clone()
    }

    pub async fn next(&mut self) -> Option<AppEvent> {
        self.rx.recv().await
    }
}

pub async fn handle_key_event(
    key: KeyEvent,
    app: &mut App,
    client: &ApiClient,
    tx: &mpsc::UnboundedSender<AppEvent>,
) {
    match app.view {
        View::TaskList => handle_task_list_input(key, app, client, tx).await,
        View::AddTask => handle_form_input(key, app, client, tx).await,
        View::ErrorDialog => handle_error_dialog_input(key, app),
    }
}

async fn handle_task_list_input(
    key: KeyEvent,
    app: &mut App,
    client: &ApiClient,
    tx: &mpsc::UnboundedSender<AppEvent>,
) {
    match key.code {
        KeyCode::Char('q') => app.quit(),
        KeyCode::Char('a') => app.show_add_form(),
        KeyCode::Char('r') => {
            app.loading = true;
            let client = client.clone();
            let tx = tx.clone();
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
        KeyCode::Down | KeyCode::Char('j') => app.next_task(),
        KeyCode::Up | KeyCode::Char('k') => app.previous_task(),
        KeyCode::Enter => {
            if let Some(task) = app.select_task() {
                let task_id = task.id;
                let client = client.clone();
                let tx = tx.clone();
                app.loading = true;
                tokio::spawn(async move {
                    match client.complete_chore(task_id).await {
                        Ok(_) => {
                            let _ = tx.send(AppEvent::TaskCompleted);
                        }
                        Err(e) => {
                            let _ = tx.send(AppEvent::Error(e.to_string()));
                        }
                    }
                });
            }
        }
        _ => {}
    }
}

async fn handle_form_input(
    key: KeyEvent,
    app: &mut App,
    client: &ApiClient,
    tx: &mpsc::UnboundedSender<AppEvent>,
) {
    match key.code {
        KeyCode::Esc => app.show_task_list(),
        KeyCode::Tab => {
            if key.modifiers.contains(KeyModifiers::SHIFT) {
                app.form_state.previous_field();
            } else {
                app.form_state.next_field();
            }
        }
        KeyCode::Enter => {
            if app.form_state.is_valid() {
                let name = app.form_state.name.clone();
                let due_date = app.form_state.due_date.clone();
                let client = client.clone();
                let tx = tx.clone();
                app.loading = true;
                tokio::spawn(async move {
                    match client.create_chore(name, due_date).await {
                        Ok(_) => {
                            let _ = tx.send(AppEvent::TaskAdded);
                        }
                        Err(e) => {
                            let _ = tx.send(AppEvent::Error(e.to_string()));
                        }
                    }
                });
            } else {
                app.show_error("Please fill in all fields with valid data".to_string());
            }
        }
        KeyCode::Backspace => {
            app.form_state.delete_char();
        }
        KeyCode::Char(c) => {
            app.form_state.insert_char(c);
        }
        _ => {}
    }
}

fn handle_error_dialog_input(key: KeyEvent, app: &mut App) {
    if matches!(key.code, KeyCode::Char(_) | KeyCode::Enter | KeyCode::Esc) {
        app.clear_error();
    }
}

pub async fn poll_events(tx: mpsc::UnboundedSender<AppEvent>) {
    loop {
        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            if let Ok(Event::Key(key)) = event::read() {
                let _ = tx.send(AppEvent::Input(key));
            }
        }
    }
}
