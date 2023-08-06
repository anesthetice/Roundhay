use axum::{
    routing::get,
    Router,
    response::Html,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { Html("<head><meta http-equiv=\"refresh\" content=\"0; URL=/home/\"/></head>") }))
        .route("/home/", get(|| async { "test" }));

    axum::Server::bind(&"0.0.0.0:1888".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
