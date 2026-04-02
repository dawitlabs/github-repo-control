use crate::models::Repo;
use reqwest::Client;
use reqwest::StatusCode;
use std::error::Error;
use std::fmt;

#[derive(serde::Deserialize)]
struct AuthenticatedUser {
    login: String,
}

#[derive(Debug)]
pub enum CredentialError {
    WrongToken,
    WrongUsername { authenticated_user: String },
    Request(Box<dyn Error>),
}

impl fmt::Display for CredentialError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CredentialError::WrongToken => write!(f, "wrong token"),
            CredentialError::WrongUsername { authenticated_user } => {
                write!(f, "wrong username; signed in as {}", authenticated_user)
            }
            CredentialError::Request(error) => write!(f, "{}", error),
        }
    }
}

impl Error for CredentialError {}

pub async fn validate_credentials(username: &str, token: &str) -> Result<(), CredentialError> {
    let client = Client::new();

    let response = client
        .get("https://api.github.com/user")
        .header("User-Agent", "github-repo-control")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|error| CredentialError::Request(Box::new(error)))?;

    if response.status() == StatusCode::UNAUTHORIZED {
        return Err(CredentialError::WrongToken);
    }

    let authenticated_user: AuthenticatedUser = response
        .error_for_status()
        .map_err(|error| CredentialError::Request(Box::new(error)))?
        .json()
        .await
        .map_err(|error| CredentialError::Request(Box::new(error)))?;

    if !authenticated_user.login.eq_ignore_ascii_case(username) {
        return Err(CredentialError::WrongUsername {
            authenticated_user: authenticated_user.login,
        });
    }

    Ok(())
}

pub async fn fetch_repos(username: &str, token: &str) -> Result<Vec<Repo>, Box<dyn Error>> {
    let client = Client::new();

    let repos: Vec<Repo> = client
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

    Ok(repos
        .into_iter()
        .filter(|repo| !repo.is_private)
        .map(|mut repo| {
            repo.selected = false;
            repo
        })
        .collect())
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
