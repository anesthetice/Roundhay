use std::net::SocketAddr;
use axum::{
    routing::get,
    Router,
    response::Html,
};

use tokio::{
    fs, io::AsyncReadExt,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { Html("<head><meta http-equiv=\"refresh\" content=\"0; URL=/home/\"/></head>") }))
        .route("/home/", get(|| async { Html("<a href='/res/vid.mp4'>download</a>") }))
        .route("/res/vid.mp4", get(test));

    axum::Server::bind(&SocketAddr::from(([0,0,0,0], 1888)))
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn test() -> Vec<u8> {
    let mut file = fs::OpenOptions::new().read(true).open("./res/test.mp4").await.unwrap();
    // convert the `AsyncRead` into a `Stream`
    let stream = ReaderStream::new(file);
    // convert the `Stream` into an `axum::body::HttpBody`
    let body = StreamBody::new(stream);

    let headers = Headers([
        (header::CONTENT_TYPE, "text/toml; charset=utf-8"),
        (
            header::CONTENT_DISPOSITION,
            "attachment; filename=\"Cargo.toml\"",
        ),
    ]);

    Ok((headers, body))
}
