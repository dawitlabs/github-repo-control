pub struct Repo{
    pub name: String,
    pub selected: bool
}

pub struct App{
    pub repos: Vec<Repo>,
    pub cursor: usize
}

impl App{
    pub fn toggle(&mut self){
        if let some(repo) = self.repos.get_mut(self.cursor){
            repo.
