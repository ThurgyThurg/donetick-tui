use crate::app::{App, FormField, View};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(1)])
        .split(f.area());

    match app.view {
        View::TaskList => {
            draw_task_list(f, chunks[0], app);
            draw_help_footer(f, chunks[1], &app.view);
        }
        View::AddTask => {
            draw_add_form(f, chunks[0], app);
            draw_help_footer(f, chunks[1], &app.view);
        }
        View::ErrorDialog => {
            draw_task_list(f, chunks[0], app);
            draw_error_dialog(f, f.area(), app);
            draw_help_footer(f, chunks[1], &app.view);
        }
    }
}

fn draw_task_list(f: &mut Frame, area: Rect, app: &App) {
    if app.loading {
        let loading = Paragraph::new("Loading tasks...")
            .block(Block::default().borders(Borders::ALL).title("Donetick Tasks"))
            .alignment(Alignment::Center);
        f.render_widget(loading, area);
        return;
    }

    if app.tasks.is_empty() {
        let empty = Paragraph::new("No tasks found. Press 'a' to add a new task.")
            .block(Block::default().borders(Borders::ALL).title("Donetick Tasks"))
            .alignment(Alignment::Center);
        f.render_widget(empty, area);
        return;
    }

    let header = Row::new(vec![
        Cell::from("Name").style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from("Due Date").style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from("Status").style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from("Priority").style(Style::default().add_modifier(Modifier::BOLD)),
    ]);

    let rows: Vec<Row> = app.tasks.iter().enumerate().map(|(i, task)| {
        let style = if i == app.selected_task {
            Style::default().bg(Color::DarkGray).fg(Color::White)
        } else {
            Style::default()
        };

        let (status_text, status_color) = match task.status {
            Some(1) => ("active", Color::Green),
            Some(2) => ("completed", Color::Blue),
            Some(0) | None => ("pending", Color::Yellow),
            Some(_) => ("unknown", Color::White),
        };

        Row::new(vec![
            Cell::from(task.name.clone()),
            Cell::from(task.next_due_date.clone().unwrap_or_else(|| "-".to_string())),
            Cell::from(status_text).style(Style::default().fg(status_color)),
            Cell::from(task.priority.map(|p| p.to_string()).unwrap_or_else(|| "-".to_string())),
        ]).style(style)
    }).collect();

    let widths = [
        Constraint::Percentage(40),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Donetick Tasks"));

    f.render_widget(table, area);
}

fn draw_add_form(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(area);

    let block = Block::default().borders(Borders::ALL).title("Add New Task");
    f.render_widget(block, area);

    let name_label = Paragraph::new("Name:");
    f.render_widget(name_label, chunks[0]);

    let name_style = if app.form_state.active_field == FormField::Name {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let name_input = Paragraph::new(app.form_state.name.clone())
        .block(Block::default().borders(Borders::ALL))
        .style(name_style);
    f.render_widget(name_input, chunks[1]);

    let due_date_label = Paragraph::new("Due Date (optional, YYYY-MM-DD):");
    f.render_widget(due_date_label, chunks[2]);

    let due_date_style = if app.form_state.active_field == FormField::DueDate {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let due_date_input = Paragraph::new(app.form_state.due_date.clone())
        .block(Block::default().borders(Borders::ALL))
        .style(due_date_style);
    f.render_widget(due_date_input, chunks[3]);
}

fn draw_error_dialog(f: &mut Frame, area: Rect, app: &App) {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Length(7),
            Constraint::Percentage(30),
        ])
        .split(area);

    let horizontal_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(popup_layout[1]);

    let error_message = app.error_message.as_ref().map(|s| s.as_str()).unwrap_or("Unknown error");

    let error_text = vec![
        Line::from(""),
        Line::from(Span::styled("Error", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(error_message),
        Line::from(""),
        Line::from(Span::styled("Press any key to continue", Style::default().fg(Color::Gray))),
    ];

    let error_block = Paragraph::new(error_text)
        .block(Block::default().borders(Borders::ALL).style(Style::default().bg(Color::Black)))
        .alignment(Alignment::Center);

    f.render_widget(error_block, horizontal_layout[1]);
}

fn draw_help_footer(f: &mut Frame, area: Rect, view: &View) {
    let help_text = match view {
        View::TaskList => {
            vec![
                Span::raw("↑/k: up | ↓/j: down | "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(": complete | "),
                Span::styled("a", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(": add | "),
                Span::styled("r", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(": refresh | "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(": quit"),
            ]
        }
        View::AddTask => {
            vec![
                Span::styled("Tab", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(": next field | "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(": submit | "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(": cancel"),
            ]
        }
        View::ErrorDialog => {
            vec![Span::raw("Press any key to continue")]
        }
    };

    let help = Paragraph::new(Line::from(help_text))
        .style(Style::default().bg(Color::DarkGray))
        .alignment(Alignment::Center);

    f.render_widget(help, area);
}
