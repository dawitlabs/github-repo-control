use crate::models::Repo;
use reqwest::Client;
use std::error::Error;

pub async fn fetch_repos(username: &str, token: &str) -> Result<Vec<Repo>, Box<dyn Error>> {
    let client = Client::new();

    let mut repos: Vec<Repo> = client
        .get(format!(
            "https://api.github.com/users/{}/repos?per_page=100",
            username
        ))
        .header("User-Agent", "github-repo-control")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?
        .json()
        .await?;

    for repo in &mut repos {
        repo.selected = false;
    }

    Ok(repos)
}

pub async fn make_private(
    username: &str,
    token: &str,
    repo: &str,
) -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    client
        .patch(format!(
            "https://api.github.com/repos/{}/{}",
            username, repo
        ))
        .header("User-Agent", "github-repo-control")
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({"private" : true}))
        .send()
        .await?;

    Ok(())
}
