use std::net::SocketAddr;
use axum::{
    routing::{get, get_service},
    Router,
    response::{Html, IntoResponse},
};

use tower_http::{
    services::ServeDir,
};

mod unit;
mod superunit;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn routes_dynamic() -> Router {
    Router::new()
        .route("/", get(|| async { Html("<head><meta http-equiv=\"refresh\" content=\"0; URL=/home/\"/></head>") }))
        .route("/home/", get(|| async { Html("<a href='/res/test.mkv' download>download</a>") }))
}

fn routes_static() -> Router {
    Router::new()
        .nest_service("/download/", get_service(ServeDir::new("./res").with_buf_chunk_size(512000)))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let routes_all = Router::new()
        .merge(routes_dynamic())
        .merge(routes_static());

    axum::Server::bind(&SocketAddr::from(([0,0,0,0], 1888)))
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
}

async fn test() -> impl IntoResponse {

}
