use axum::{routing::get, response::Html, Router};
use askama::{Template};

pub async fn serve_web() {
    serve().await
}

#[derive(Template)]
#[template(path = "pr.html")]
struct Test {}

async fn serve() {

    let template = Test {};
    
    let app = Router::new()
        .route("/pr", get(Html(template.render().unwrap())))
        .route("/", get("Hi"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
	.unwrap();
    tracing::info!("Serving web on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap()
}
