use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Repo {
    pub name: String,

    #[serde(default, rename = "private")]
    pub is_private: bool,

    #[serde(skip)]
    pub selected: bool,
}
