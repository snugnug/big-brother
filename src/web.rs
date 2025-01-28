use crate::github;
use reqwest::Client;
use askama::Template;
use axum::{extract::Path, response::Html, routing::get, Router};

pub async fn serve_web() {
    serve().await
}

#[derive(Template)]
#[template(path = "pr.html")]
struct Test {
    test: String,
    error: String,
    id: u64,
    failed: bool,
    branches: Vec<String>,
    merged_into: Vec<bool>
}

async fn get_pr(Path(prId): Path<u64>) -> Html<String> {

    let client = Client::builder()
        .user_agent(format!("big-brother {}", env!("CARGO_PKG_VERSION")))
        .build()
        .unwrap();


    let pr = match github::get_pr_info(client, prId).await {
	Ok(data) => {
	    tracing::debug!("Got pr {}", prId);
	    data
	}
	Err(err) => {
	    tracing::error!("Failed to get pr, {}", err);
	    let template = Test {
		test: "Bruhh epic fail".to_string(),
		id: prId,
		failed: true,
		error: err.to_string(),
		branches: vec![],
		merged_into: vec![]
	    };
	    return Html(template.render().unwrap());
	}
    };
        
    let template = Test {
        test: "Hello".to_string(),
        id: prId,
	failed: false,
	error: "You shouldn't see this lol".to_string(),
	branches: target_branches,
	merged_into: in_branches
    };

    return Html(template.render().unwrap());
}

async fn serve() {
    let app = Router::new()
        .route("/pr/{id}", get(get_pr))
        .route("/", get("Hi"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::info!("Serving web on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap()
}
