use crate::models::Repo;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    Credentials,
    Loading,
    Dashboard,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CredentialField {
    Username,
    Token,
}

pub struct App {
    pub repos: Vec<Repo>,
    pub cursor: usize,
    pub status_message: Option<String>,
    pub mode: AppMode,
    pub active_field: CredentialField,
    pub username_input: String,
    pub token_input: String,
}

impl App {
    pub fn new(repos: Vec<Repo>) -> Self {
        Self {
            repos,
            cursor: 0,
            status_message: None,
            mode: AppMode::Credentials,
            active_field: CredentialField::Username,
            username_input: String::new(),
            token_input: String::new(),
        }
    }

    pub fn set_status<S: Into<String>>(&mut self, message: S) {
        self.status_message = Some(message.into());
    }

    pub fn clear_status(&mut self) {
        self.status_message = None;
    }

    pub fn credential_value(&self, field: CredentialField) -> &str {
        match field {
            CredentialField::Username => &self.username_input,
            CredentialField::Token => &self.token_input,
        }
    }

    pub fn credential_value_mut(&mut self, field: CredentialField) -> &mut String {
        match field {
            CredentialField::Username => &mut self.username_input,
            CredentialField::Token => &mut self.token_input,
        }
    }

    pub fn masked_token(&self) -> String {
        "•".repeat(self.token_input.chars().count())
    }

    pub fn next_credential_field(&mut self) {
        self.active_field = match self.active_field {
            CredentialField::Username => CredentialField::Token,
            CredentialField::Token => CredentialField::Token,
        };
    }

    pub fn push_credential_char(&mut self, ch: char) {
        self.credential_value_mut(self.active_field).push(ch);
    }

    pub fn pop_credential_char(&mut self) {
        self.credential_value_mut(self.active_field).pop();
    }

    pub fn credentials_ready(&self) -> bool {
        !self.username_input.trim().is_empty() && !self.token_input.trim().is_empty()
    }

    pub fn username(&self) -> &str {
        self.username_input.trim()
    }

    pub fn token(&self) -> &str {
        self.token_input.trim()
    }

    pub fn total_count(&self) -> usize {
        self.repos.len()
    }

    pub fn selected_count(&self) -> usize {
        self.repos.iter().filter(|repo| repo.selected).count()
    }

    pub fn current_repo(&self) -> Option<&Repo> {
        self.repos.get(self.cursor)
    }

    pub fn next(&mut self) {
        if self.repos.is_empty() {
            return;
        }

        self.cursor = (self.cursor + 1) % self.repos.len();
    }

    pub fn previous(&mut self) {
        if self.repos.is_empty() {
            return;
        }

        if self.cursor == 0 {
            self.cursor = self.repos.len() - 1;
        } else {
            self.cursor -= 1;
        }
    }

    pub fn toggle(&mut self) {
        if let Some(repo) = self.repos.get_mut(self.cursor) {
            repo.selected = !repo.selected;
        }
    }

    pub fn select_all(&mut self) {
        for repo in &mut self.repos {
            repo.selected = true;
        }
    }

    pub fn unselect_all(&mut self) {
        for repo in &mut self.repos {
            repo.selected = false;
        }
    }

    pub fn has_selection(&self) -> bool {
        self.repos.iter().any(|repo| repo.selected)
    }

    pub fn selected_repos(&self) -> Vec<String> {
        self.repos
            .iter()
            .filter(|r| r.selected)
            .map(|r| r.name.clone())
            .collect()
    }
}
