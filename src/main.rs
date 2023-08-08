use std::net::SocketAddr;
use axum::{
    routing::{get, get_service},
    Router,
    response::{Html, IntoResponse},
};
use superunit::Superunit;
use tower_http::{
    services::ServeDir,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod unit;
mod superunit;
mod print;

fn routes_dynamic() -> Router {
    Router::new()
        .route("/", get(|| async { Html("<head><meta http-equiv=\"refresh\" content=\"0; URL=/home/\"/></head>") }))
        .route("/home/", get(get(dynamic_handler)))
}

fn routes_static() -> Router {
    Router::new()
        .nest_service("/download/", get_service(ServeDir::new("./res").with_buf_chunk_size(512000)))
}

#[tokio::main]
async fn main() -> () {

    #[cfg(feature = "print-unit")]
    print::print_unit();
    #[cfg(feature = "print-unit")]
    return;

    #[cfg(feature = "print-superunit")]
    print::print_superunit();
    #[cfg(feature = "print-superunit")]
    return;

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

async fn dynamic_handler() -> impl IntoResponse {
    let html_head: String = "
    <!DOCTYPE html>
    <html lang=\"en\">
    <head>
        <meta charset=\"UTF-8\">
        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
        <meta name=\"description\" content=\"server\">
        <meta name=\"author\" content=\"anesthetice/oether\">
        <title>Roundhay 1.0.0</title>
        <style>
        table {
          width: 100%;
          border-collapse: collapse;
          border: 1px solid #ccc;
        }
        th, td {
          padding: 8px;
          text-align: left;
          border-bottom: 1px solid #ccc;
        }
        th {
          background-color: #f2f2f2;
          font-weight: bold;
        }
        tbody tr:nth-child(even) {
          background-color: #f2f2f2;
        }
      </style>
    </head>
    <body>".to_string();

    let html_table = Superunit::load().await.to_html_string();

    Html(format!("{}{}</body></html>", html_head, html_table))
}
