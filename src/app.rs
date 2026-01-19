use crate::api::Chore;

pub struct App {
    pub running: bool,
    pub view: View,
    pub tasks: Vec<Chore>,
    pub selected_task: usize,
    pub form_state: FormState,
    pub loading: bool,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum View {
    TaskList,
    AddTask,
    ErrorDialog,
}

pub struct FormState {
    pub name: String,
    pub due_date: String,
    pub cursor_position: usize,
    pub active_field: FormField,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FormField {
    Name,
    DueDate,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            view: View::TaskList,
            tasks: Vec::new(),
            selected_task: 0,
            form_state: FormState::new(),
            loading: true,
            error_message: None,
        }
    }

    pub fn next_task(&mut self) {
        if !self.tasks.is_empty() {
            self.selected_task = (self.selected_task + 1) % self.tasks.len();
        }
    }

    pub fn previous_task(&mut self) {
        if !self.tasks.is_empty() {
            if self.selected_task == 0 {
                self.selected_task = self.tasks.len() - 1;
            } else {
                self.selected_task -= 1;
            }
        }
    }

    pub fn select_task(&self) -> Option<&Chore> {
        self.tasks.get(self.selected_task)
    }

    pub fn show_add_form(&mut self) {
        self.view = View::AddTask;
        self.form_state = FormState::new();
    }

    pub fn show_task_list(&mut self) {
        self.view = View::TaskList;
        self.loading = true;
    }

    pub fn show_error(&mut self, message: String) {
        self.error_message = Some(message);
        self.view = View::ErrorDialog;
        self.loading = false;
    }

    pub fn clear_error(&mut self) {
        self.error_message = None;
        self.view = View::TaskList;
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}

impl FormState {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            due_date: String::new(),
            cursor_position: 0,
            active_field: FormField::Name,
        }
    }

    pub fn insert_char(&mut self, c: char) {
        match self.active_field {
            FormField::Name => {
                self.name.insert(self.cursor_position, c);
                self.cursor_position += 1;
            }
            FormField::DueDate => {
                self.due_date.insert(self.cursor_position, c);
                self.cursor_position += 1;
            }
        }
    }

    pub fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            match self.active_field {
                FormField::Name => {
                    self.name.remove(self.cursor_position - 1);
                }
                FormField::DueDate => {
                    self.due_date.remove(self.cursor_position - 1);
                }
            }
            self.cursor_position -= 1;
        }
    }

    pub fn next_field(&mut self) {
        match self.active_field {
            FormField::Name => {
                self.active_field = FormField::DueDate;
                self.cursor_position = self.due_date.len();
            }
            FormField::DueDate => {
                self.active_field = FormField::Name;
                self.cursor_position = self.name.len();
            }
        }
    }

    pub fn previous_field(&mut self) {
        self.next_field();
    }

    pub fn is_valid(&self) -> bool {
        // Only require name, due date is optional
        if self.name.is_empty() {
            return false;
        }

        // If due date is provided, validate it
        if !self.due_date.is_empty() {
            return self.is_valid_date();
        }

        true
    }

    fn is_valid_date(&self) -> bool {
        if self.due_date.is_empty() {
            return true; // Empty is valid (optional)
        }

        let parts: Vec<&str> = self.due_date.split('-').collect();
        if parts.len() != 3 {
            return false;
        }

        let year = parts[0].parse::<i32>();
        let month = parts[1].parse::<u32>();
        let day = parts[2].parse::<u32>();

        if year.is_err() || month.is_err() || day.is_err() {
            return false;
        }

        let month = month.unwrap();
        let day = day.unwrap();

        month >= 1 && month <= 12 && day >= 1 && day <= 31
    }
}
