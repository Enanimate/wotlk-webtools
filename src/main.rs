use axum::{
    body::{Body, HttpBody, StreamBody}, http::{header::{self, CONTENT_TYPE}, HeaderMap, HeaderValue, Method, StatusCode}, response::IntoResponse, routing::{get, post}, Json, Router
};
use serde::{Deserialize, Serialize};
use serde_json::{de::Read, json, Value};
use tokio::fs;
use tokio_util::io::ReaderStream;
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

async fn downloadleg() -> impl IntoResponse {
    println!("download requested");

    //"../wotlk-client-file/wotlk-client.zip"
    let filepath = path::Path::new("testpayload.txt");

    fs::read(filepath).await.unwrap_or(Vec::new()).into_response()
}

async fn download() -> impl IntoResponse {
    println!("Download Requested...");

    let file = match fs::File::open("../wotlk-client-file/wotlk-client.zip").await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };
    
    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE, 
        "text/plain; charset=utf-8".parse().unwrap(),
    );
    headers.insert(
        header::CONTENT_DISPOSITION, 
        "attachment; filename=\"download_file.txt\"".parse().unwrap(),
    );
    headers.insert(
        header::CONTENT_LENGTH, 
        HeaderValue::from(body.size_hint().exact().unwrap()),
    );

    Ok((headers, body))
}