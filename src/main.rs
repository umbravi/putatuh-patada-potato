mod domain;
mod request;

use crate::request::weather_request;
use axum::{routing::get, Router};
use std::net::SocketAddr;

async fn index() -> &'static str {
    "Index"
}

async fn stats() -> &'static str {
    "Stats"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/api/weather", get(weather_request))
        .route("/stats", get(stats));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
