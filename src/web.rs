use axum::{routing::get, response::Html, Router, extract::Path};
use askama::{Template};

pub async fn serve_web() {
    serve().await
}

#[derive(Template)]
#[template(path = "pr.html")]
struct Test {
    test: String,
    id: u64
}

async fn get_pr(Path(prId): Path<u64>) -> Html<String>{

    let template = Test {
	test: "Hello".to_string(),
	id: prId
    };
    
    return Html(template.render().unwrap())
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
