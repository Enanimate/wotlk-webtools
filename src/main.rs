use axum::{
    http::{header::CONTENT_TYPE, HeaderValue, Method}, response::IntoResponse, routing::{get, post}, Json, Router
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::fs;
use std::{net::SocketAddr, path};
use tower_http::cors::CorsLayer;

#[derive(Deserialize)]
pub struct Login {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    validlogin: bool,
    uuid: u16,
    admin: bool,
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin("http://127.0.0.1:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE]);

    let app = Router::new()
        .route("/", post(jsonfn))
        .route("/download", get(download))
        .layer(cors);

    let addr = SocketAddr::from(([10, 0, 1, 243], 3001));
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn jsonfn(Json(payload): Json<Login>) -> Json<LoginResponse> {
    println!("flag");
    if payload.username == "admin" && payload.password == "password" {
        let body = LoginResponse {
            validlogin: true,
            uuid: todo!(),
            admin: todo!(),
        };
        Json(body)
    } else {
        let body = LoginResponse {
            validlogin: false,
            uuid: todo!(),
            admin: todo!(),
        };
        Json(body)
    }
}

async fn download() -> impl IntoResponse {
    println!("download requested");
    let filepath = path::Path::new("../wotlk-client-file/wotlk-client.zip");

    fs::read(filepath).await.unwrap_or(Vec::new()).into_response()
}