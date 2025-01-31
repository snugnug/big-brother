use crate::github;
use askama::Template;
use axum::{extract::Path, response::Html, routing::get, Router};
use reqwest::Client;
use tower_http::services::ServeFile;

pub async fn serve_web() {
    serve().await
}

#[derive(Template)]
#[template(path = "pr.html")]
struct Test {
    pr_title: String,
    error: String,
    failed: bool,
    branches: Vec<String>,
    merged_into: Vec<bool>,
}

async fn get_pr(Path(prId): Path<u64>) -> Html<String> {
    let client = Client::builder()
        .user_agent(format!("big-brother {}", env!("CARGO_PKG_VERSION")))
             .build()
        .unwrap();

    let pr = match github::get_pr_info(client.clone(), prId).await {
        Ok(data) => {
            tracing::debug!("Got pr {}", prId);
            data
        }
        Err(err) => {
            tracing::error!("Failed to get pr, {}", err);
            let template = Test {
                pr_title: "Errored!".to_string(),
                failed: true,
                error: err.to_string(),
                branches: vec![],
                merged_into: vec![],
            };
            return Html(template.render().unwrap());
        }
    };

    let target_branches = vec![
        "master".to_string(),
        "nixpkgs-unstable".to_string(),
        "nixos-unstable-small".to_string(),
        "nixos-unstable".to_string(),
        "nixos-24.11-small".to_string(),
        "nixos-24.11".to_string(),
    ];

    let mut in_branches: Vec<bool> = vec![];

    let tasks: Vec<_> = target_branches
        .clone()
        .iter()
        .map(|branch| {
            let branch_clone = branch.clone();
            let client_clone = client.clone();
            let pr_merge_commit_sha = pr.merge_commit_sha.clone();
            tokio::spawn(async move {
                github::compare_branches_api(client_clone, branch_clone, pr_merge_commit_sha).await
            })
        })
        .collect();

    for task in tasks {
        match task.await {
            Ok(data) => in_branches.push(data.unwrap()),
            Err(err) => {
                tracing::error!("Failed to get pr, {}", err);
                let template = Test {
                    pr_title: "Errored!".to_string(),
                    failed: true,
                    error: err.to_string(),
                    branches: vec![],
                    merged_into: vec![],
                };
                return Html(template.render().unwrap());
            }
        };
    }

    if in_branches.is_empty() || in_branches.is_empty() {
        tracing::warn!("Results or in_branches are empty, check logic.");
    }

    let template = Test {
        pr_title: pr.title,
        failed: false,
        error: "You shouldn't see this lol".to_string(),
        branches: target_branches,
        merged_into: in_branches,
    };

    return Html(template.render().unwrap());
}

async fn serve() {
    let app = Router::new()
        .route("/pr/{id}", get(get_pr))
        .route("/", get("Hi"))
        .route_service("/main.css", ServeFile::new("assets/main.css"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::info!("Serving web on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap()
}
