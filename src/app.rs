use crate::models::Repo;

pub struct App {
    pub repos: Vec<Repo>,
    pub cursor: usize,
}

impl App {
    pub fn new(repos: Vec<Repo>) -> Self {
        Self { repos, cursor: 0 }
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

    pub fn selected_repos(&self) -> Vec<String> {
        self.repos
            .iter()
            .filter(|r| r.selected)
            .map(|r| r.name.clone())
            .collect()
    }
}
