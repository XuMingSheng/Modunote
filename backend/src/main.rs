use axum::{
    routing::get,
    Router,
};

async fn ping() -> &'static str {
    "pong"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/ping", get(ping));
    
    let addr = "0.0.0.0:8080";

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    
    axum::serve(listener, app)
        .await
        .unwrap();
    
    println!("Backend listening on http://{}", addr);
}
