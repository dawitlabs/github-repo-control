use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Repo {
    pub name: String,

    #[serde(skip)]
    pub selected: bool,
}
