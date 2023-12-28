use axum::{
    response::{Html, IntoResponse},
    routing::{get, get_service},
    Router,
    extract::Query,
};
use tokio::net::TcpListener;
use std::{net::SocketAddr, collections::HashMap};
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
        .route("/home/", get(get(home_dynamic_handler)))
        .route("/stream/", get(get(stream_dynamic_handler)))
}

fn routes_static() -> Router {
    Router::new().nest_service(
        "/res/",
        get_service(
          ServeDir::new("./res")
            .with_buf_chunk_size(2097152)
          ),
    )
}

#[tokio::main]
async fn main() {
    #[cfg(feature = "print")]
    print::print();
    #[cfg(feature = "print")]
    return;

    let routes_all = Router::new().merge(routes_dynamic()).merge(routes_static());

    let listener = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], 1888)))
      .await
      .unwrap();

    axum::serve(listener, routes_all)
      .await
      .unwrap();
}

async fn home_dynamic_handler() -> impl IntoResponse {
    println!("[INFO] /home/ handler called");
    let html_head: String = indoc::formatdoc! {
      "<!DOCTYPE html>
      <html lang=\"en\">
      <head>
          <meta charset=\"UTF-8\">
          <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
          <meta name=\"description\" content=\"server\">
          <meta name=\"author\" content=\"anesthetice\">
          <title>Roundhay 1.2.0</title>
          <style>
              table {{
                  width: 100%;
                  border-collapse: collapse;
                  border: thin solid #ccc;
              }}
              th, td {{
                  padding: 8px;
                  text-align: left;
                  border-bottom: 1px solid #ccc;
              }}
              th {{
                  background-color: #f2f2f2;
                  font-weight: bold;
              }}
              .hidden {{
                  display: none;
              }}
              tr:hover {{background-color: #D6EEEE;}}
          </style>
      </head>
      <body>"
    };

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

    let html_table = Superunit::load().as_html_string();

    Html(format!(
        "{}{}{}</body>\n</html>",
        html_head, html_table, javascript
    ))
}

async fn stream_dynamic_handler(Query(params): Query<HashMap<String, String>> ) -> impl IntoResponse {
    if let Some(link) = params.get("source") {
        Html(indoc::formatdoc! {
            "<!DOCTYPE html>
            <html lang=\"en\">
            <head>
                <meta charset=\"UTF-8\">
                <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
                <meta name=\"description\" content=\"server\">
                <meta name=\"author\" content=\"anesthetice\">
                <title>Roundhay 1.2.0</title>
            </head>
            <body style=\"background-color:#000000;\">
                <center>
                <video controls>
                    <source src=\"/res/{}\" type=\"video/mp4\">
                    unable to play the video
                </video>
                </center>
            </body>
            </html>", 
            link
        })
    } else {
        Html("Error 404".to_string())
    }
}
