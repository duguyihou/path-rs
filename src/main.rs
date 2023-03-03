use std::net::SocketAddr;

use axum::{response::Html, routing::get, Router, Server};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index_handler));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index_handler() -> Html<&'static str> {
    Html("Hello world")
}
