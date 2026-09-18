use axum::{Router, response::Html, routing::get};
use std::env;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let bind = format!("{}:{}", host, port);

    let listener = tokio::net::TcpListener::bind(&bind).await.unwrap();
    println!("Server listening on http://{}", bind);
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Html<&'static str> {
    Html("<h1>Inicio</h1>")
}
