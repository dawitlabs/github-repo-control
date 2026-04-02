pub struct Repo {
    pub name: String,
    pub selected: bool,
}

pub struct App {
    pub repos: Vec<Repo>,
    pub cursor: usize,
}

impl App {
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
