use axum::{routing::get, Router};

pub async fn serve_web() {
    serve().await
}

async fn serve() {
    let app = Router::new()
        .route("/", get("Hi"));;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
	.unwrap();
    tracing::info!("Serving web on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap()
}
