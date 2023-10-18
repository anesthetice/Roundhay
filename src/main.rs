use axum::{
    response::{Html, IntoResponse},
    routing::{get, get_service},
    Router,
};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

mod print;
mod superunit;
mod traits;
mod unit;

use crate::traits::WebContent;
use superunit::Superunit;

fn routes_dynamic() -> Router {
    Router::new()
        .route(
            "/",
            get(|| async {
                Html("<head><meta http-equiv=\"refresh\" content=\"0; URL=/home/\"/></head>")
            }),
        )
        .route("/home/", get(get(dynamic_handler)))
}

fn routes_static() -> Router {
    Router::new().nest_service(
        "/download/",
        get_service(ServeDir::new("./res").with_buf_chunk_size(512000)),
    )
}

#[tokio::main]
async fn main() {
    #[cfg(feature = "print")]
    print::print();
    #[cfg(feature = "print")]
    return;

    let routes_all = Router::new().merge(routes_dynamic()).merge(routes_static());

    axum::Server::bind(&SocketAddr::from(([0, 0, 0, 0], 1888)))
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
}

async fn dynamic_handler() -> impl IntoResponse {
    println!("[INFO] /home/ handler called");
    let html_head: String = "
    <!DOCTYPE html>
    <html lang=\"en\">
    <head>
        <meta charset=\"UTF-8\">
        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
        <meta name=\"description\" content=\"server\">
        <meta name=\"author\" content=\"anesthetice\">
        <title>Roundhay 1.1.0</title>
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
            background-color: #d0d3d9;
          }
          .hidden {
            display: none;
          }
        </style>
    </head>
    <body>
    "
    .to_string();

    let javascript: String = "
    <script>
      function showhideElements(id) {
        var elements = document.querySelectorAll(`[id='${id}']`);
        for (var i = 0; i < elements.length; i++) {
          if (elements[i].classList.contains('hidden')) {
            elements[i].classList.remove('hidden');
          } else {
            elements[i].classList.add('hidden');
          }
        }
      }
    </script>
    "
    .to_string();

    let html_table = Superunit::load().await.as_html_string();

    Html(format!(
        "{}{}{}</body>\n</html>",
        html_head, html_table, javascript
    ))
}
