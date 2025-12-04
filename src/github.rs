use reqwest::{Client, RequestBuilder, StatusCode};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::time::{Duration, Instant};

// Define repository type for flexibility
#[derive(Debug, Clone)]
pub struct Repository {
    pub owner: String,
    pub name: String,
}

impl Default for Repository {
    fn default() -> Self {
        Self {
            owner: "nixos".to_string(),
            name: "nixpkgs".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum GithubError {
    RequestFailed(reqwest::Error),
    RateLimitExceeded,
    ApiError { status: StatusCode, message: String },
    SerializationError(serde_json::Error),
}

impl fmt::Display for GithubError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequestFailed(e) => write!(f, "API request failed: {}", e),
            Self::RateLimitExceeded => write!(f, "Rate limit exceeded"),
            Self::ApiError { status, message } => {
                write!(f, "API returned error: {} - {}", status, message)
            }
            Self::SerializationError(e) => write!(f, "Serialization error: {}", e),
        }
    }
}

impl std::error::Error for GithubError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::RequestFailed(e) => Some(e),
            Self::SerializationError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for GithubError {
    fn from(err: reqwest::Error) -> Self {
        Self::RequestFailed(err)
    }
}

impl From<serde_json::Error> for GithubError {
    fn from(err: serde_json::Error) -> Self {
        Self::SerializationError(err)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrInfo {
    pub title: String,
    pub id: u64,
    pub state: String,
    pub merged: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_commit_sha: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrCompare {
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RateLimit {
    resources: RateLimitResources,
}

#[derive(Serialize, Deserialize, Debug)]
struct RateLimitResources {
    core: RateLimitData,
}

#[derive(Serialize, Deserialize, Debug)]
struct RateLimitData {
    limit: u32,
    remaining: u32,
    reset: u64,
}

// Static rate limit tracking for unauthenticated requests
static mut LAST_REQUEST: Option<Instant> = None;
const UNAUTHENTICATED_DELAY_MS: u64 = 1000; // 1 second between requests

fn authorize_request(request: RequestBuilder) -> RequestBuilder {
    match std::env::var("GITHUB_API_KEY") {
        Ok(token) if !token.is_empty() => {
            request.header(reqwest::header::AUTHORIZATION, format!("Bearer {}", token))
        }
        _ => request,
    }
}

async fn handle_rate_limit(client: &Client, authenticated: bool) -> Result<(), GithubError> {
    if authenticated {
        // For authenticated requests, check the rate limit via API
        let request = authorize_request(client.get("https://api.github.com/rate_limit"));
        let response = request.send().await?;

        if response.status() == StatusCode::OK {
            let rate_limit: RateLimit = response.json().await?;
            let remaining = rate_limit.resources.core.remaining;

            if remaining <= 5 {
                let reset_time = rate_limit.resources.core.reset as u64;
                let current_time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();

                if reset_time > current_time {
                    let wait_time = reset_time - current_time + 1;
                    tracing::warn!(
                        "Rate limit nearly exceeded ({}). Waiting {} seconds",
                        remaining,
                        wait_time
                    );
                    tokio::time::sleep(Duration::from_secs(wait_time)).await;
                }
            }
        }
    } else {
        // For unauthenticated requests, use a simple delay
        unsafe {
            if let Some(last_time) = LAST_REQUEST {
                let elapsed = last_time.elapsed();
                if elapsed < Duration::from_millis(UNAUTHENTICATED_DELAY_MS) {
                    let wait_time = Duration::from_millis(UNAUTHENTICATED_DELAY_MS) - elapsed;
                    tracing::debug!("Rate limiting: waiting {:?} before next request", wait_time);
                    tokio::time::sleep(wait_time).await;
                }
            }
            LAST_REQUEST = Some(Instant::now());
        }
    }

    Ok(())
}

pub async fn get_pr_info(
    client: &Client,
    pr: u64,
    repo: Option<&Repository>,
) -> Result<PrInfo, GithubError> {
    let default_repo = Repository::default();
    let repo = repo.unwrap_or(&default_repo);

    let authenticated = std::env::var("GITHUB_API_KEY").is_ok();

    // Even when we *do* have an API key, GitHub has rate limits to the number of
    // requests you can do. If you use Nix, you are likely to hit those often because
    // you will normally be sending a lot of requests. Let's handle rate limits by
    // postponing the task for a while if we hit a rate limit instead of panicking.
    handle_rate_limit(client, authenticated).await?;

    tracing::info!("Fetching PR #{} from {}/{}", pr, repo.owner, repo.name);
    let url = format!(
        "https://api.github.com/repos/{}/{}/pulls/{}",
        repo.owner, repo.name, pr
    );

    let request = authorize_request(client.get(&url));
    let response = request.send().await?;

    tracing::debug!("PR #{} response status: {}", pr, response.status());

    match response.status() {
        StatusCode::OK => {
            let pr_info = response.json::<PrInfo>().await?;
            tracing::debug!(
                pr_id = pr_info.id,
                pr_title = pr_info.title,
                "PR info retrieved successfully"
            );
            Ok(pr_info)
        }
        StatusCode::FORBIDDEN | StatusCode::TOO_MANY_REQUESTS => {
            tracing::error!("Rate limit exceeded when fetching PR #{}", pr);
            Err(GithubError::RateLimitExceeded)
        }
        status => {
            let message = response
                .text()
                .await
                .unwrap_or_else(|_| "No error message".to_string());
            tracing::error!("Failed to fetch PR #{}: Status {}: {}", pr, status, message);
            Err(GithubError::ApiError { status, message })
        }
    }
}

pub async fn compare_branches_api(
    client: &Client,
    branch: &str,
    commit_hash: &str,
    repo: Option<&Repository>,
) -> Result<bool, GithubError> {
    let default_repo = Repository::default();
    let repo = repo.unwrap_or(&default_repo);

    let authenticated = std::env::var("GITHUB_API_KEY").is_ok();
    handle_rate_limit(client, authenticated).await?;

    tracing::info!(
        "Comparing commit {} with branch {} in {}/{}",
        commit_hash,
        branch,
        repo.owner,
        repo.name
    );

    let url = format!(
        "https://api.github.com/repos/{}/{}/compare/{}...{}",
        repo.owner, repo.name, branch, commit_hash
    );

    let request = authorize_request(client.get(&url));
    let response = request.send().await?;

    tracing::debug!(
        "Compare response status: {}, URL: {}",
        response.status(),
        url
    );

    match response.status() {
        StatusCode::OK => {
            let output = response.json::<PrCompare>().await?;
            let is_in_nixpkgs = output.status == "behind" || output.status == "identical";

            tracing::info!(
                "Commit {} is {} branch {}",
                commit_hash,
                if is_in_nixpkgs { "in" } else { "not in" },
                branch
            );

            Ok(is_in_nixpkgs)
        }
        StatusCode::FORBIDDEN | StatusCode::TOO_MANY_REQUESTS => {
            tracing::error!("Rate limit exceeded when comparing branches");
            Err(GithubError::RateLimitExceeded)
        }
        status => {
            let message = response
                .text()
                .await
                .unwrap_or_else(|_| "No error message".to_string());
            tracing::error!("Failed to compare branches: Status {}: {}", status, message);
            Err(GithubError::ApiError { status, message })
        }
    }
}
