use axum::{routing::get, Router};
use std::net::SocketAddr;

async fn index() -> &'static str {
    "Hello, Axum!"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    
    println!("\nlistening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}