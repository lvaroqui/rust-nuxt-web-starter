use std::net::SocketAddr;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct User {
    id: u32,
    username: String,
}

#[tokio::main]
async fn main() {
    let addr = {
        let port = std::env::var("PORT")
            .expect("PORT env var should be set")
            .parse()
            .expect("PORT env is not valid");
        SocketAddr::from(([0, 0, 0, 0], port))
    };

    let api_routes = Router::new()
        .route("/", get(|| async { "Hello from BackEnd!" }))
        .route("/user", get(get_user));
    let app = Router::new().nest("/api", api_routes);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_user() -> impl IntoResponse {
    let user = User {
        id: 1337,
        username: "toto".to_string(),
    };

    (StatusCode::OK, Json(user))
}
