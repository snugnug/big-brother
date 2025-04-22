use crate::github::{self, Repository};
use askama::Template;
use axum::{
    Router,
    extract::Path,
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
};
use reqwest::Client;
use std::{net::Ipv4Addr, sync::Arc};
use tower_http::services::ServeFile;
use tracing::{error, info, warn};

#[derive(Template)]
#[template(path = "pr.html")]
struct PullRequest {
    pr_title: String,
    error: String,
    failed: bool,
    closed: bool,
    branches: Vec<String>,
    merged_into: Vec<bool>,
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index {}

// Define a default set of branches to check
const DEFAULT_BRANCHES: [&str; 6] = [
    "master",
    "nixpkgs-unstable",
    "nixos-unstable-small",
    "nixos-unstable",
    "nixos-24.11-small",
    "nixos-24.11",
];

// Error response helper
fn render_error(title: &str, message: &str) -> Response {
    let template = PullRequest {
        pr_title: title.to_string(),
        failed: true,
        closed: false,
        error: message.to_string(),
        branches: vec![],
        merged_into: vec![],
    };
    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            error!("Template rendering failed: {}", e);
            Html(format!("<h1>Internal Server Error</h1><p>{}</p>", e)).into_response()
        }
    }
}

async fn get_pr(Path(pr_id): Path<u64>, client: axum::extract::Extension<Arc<Client>>) -> Response {
    info!("Processing request for PR #{}", pr_id);

    // Use a shared repo configuration
    let repo = Repository::default();

    // Get PR info
    let pr = match github::get_pr_info(&client, pr_id, Some(&repo)).await {
        Ok(data) => data,
        Err(err) => {
            error!("Failed to get PR #{}: {}", pr_id, err);
            return render_error("Error Retrieving PR", &err.to_string());
        }
    };

    // Handle closed, non-merged PRs
    if !pr.merged && pr.state == "closed" {
        let template = PullRequest {
            pr_title: pr.title,
            failed: false,
            closed: true,
            error: String::new(),
            branches: vec![],
            merged_into: vec![],
        };

        return match template.render() {
            Ok(html) => Html(html).into_response(),
            Err(e) => render_error("Template Error", &e.to_string()),
        };
    }

    // Ensure we have a merge commit SHA
    let merge_commit_sha = match &pr.merge_commit_sha {
        Some(sha) => sha,
        None => {
            return render_error(
                "Invalid PR Data",
                "PR is marked as merged but has no merge commit SHA",
            );
        }
    };

    // Create vectors of branches to check
    let target_branches: Vec<String> = DEFAULT_BRANCHES.iter().map(|&s| s.to_string()).collect();

    // Check which branches contain the merge commit
    let mut futures = Vec::new();

    for branch in &target_branches {
        let client = client.clone();
        let branch = branch.clone();
        let sha = merge_commit_sha.clone();
        let repo = repo.clone();

        futures.push(async move {
            match github::compare_branches_api(&client, &branch, &sha, Some(&repo)).await {
                Ok(is_in_branch) => Ok(is_in_branch),
                Err(err) => {
                    error!("Failed to check branch {}: {}", branch, err);
                    Err(err)
                }
            }
        });
    }

    // Execute all API checks concurrently
    let results = futures::future::join_all(futures).await;

    // Process results
    let mut in_branches = Vec::new();
    for result in results {
        match result {
            Ok(is_in_branch) => in_branches.push(is_in_branch),
            Err(err) => {
                return render_error("Branch Check Failed", &err.to_string());
            }
        }
    }

    if in_branches.is_empty() {
        warn!("Branch check yielded no results for PR #{}", pr_id);
    }

    // Render successful template
    let template = PullRequest {
        pr_title: pr.title,
        failed: false,
        closed: false,
        error: String::new(),
        branches: target_branches,
        merged_into: in_branches,
    };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => render_error("Template Error", &e.to_string()),
    }
}

async fn index() -> Response {
    let template = Index {};
    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => render_error("Template Error", &e.to_string()),
    }
}

pub async fn serve(host: Ipv4Addr, port: u16) {
    // Create a shared client with appropriate headers
    let client = Client::builder()
        .user_agent(format!("big-brother {}", env!("CARGO_PKG_VERSION")))
        .build()
        .unwrap_or_else(|e| {
            error!("Failed to build HTTP client: {}", e);
            std::process::exit(1);
        });

    let shared_client = Arc::new(client);

    let app = Router::new()
        .route("/pr/{id}", get(get_pr))
        .route("/pr/", get(|| async { Redirect::permanent("/") }))
        .route("/", get(index))
        .route_service("/main.css", ServeFile::new("assets/main.css"))
        .layer(axum::extract::Extension(shared_client));

    let addr = format!("{}:{}", host, port);
    match tokio::net::TcpListener::bind(&addr).await {
        Ok(listener) => {
            info!(
                "Serving web on {}",
                listener
                    .local_addr()
                    .unwrap_or_else(|_| { format!("{}", addr).parse().unwrap() })
            );

            if let Err(e) = axum::serve(listener, app).await {
                error!("Server error: {}", e);
                std::process::exit(1);
            }
        }
        Err(e) => {
            error!("Failed to bind to {}: {}", addr, e);
            std::process::exit(1);
        }
    }
}
