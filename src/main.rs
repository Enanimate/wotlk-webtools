use axum::{
    handler::HandlerWithoutStateExt, http::{header::{self, CONTENT_TYPE}, uri::Authority, HeaderMap, HeaderValue, Method, StatusCode, Uri}, response::{IntoResponse, Redirect}, routing::get, BoxError, Json, Router
};
use axum_extra::extract::Host;
use axum_server::tls_rustls::RustlsConfig;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::{types::chrono::{self, Local}, MySql, MySqlPool, Pool, Row};
use tokio::fs;
use tokio_util::io::ReaderStream;
use std::{fmt::Debug, net::{IpAddr, SocketAddr}, path::PathBuf};
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone, Copy)]
struct Ports {
    http: u16,
    https: u16,
}

#[derive(Deserialize)]
pub struct Configs {
    db_user: String,
    db_password: String,
}

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
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let ports = Ports {
        http: 7878,
        https: 3001,
    };

    tokio::spawn(redirect_http_to_https(ports));

    let config = RustlsConfig::from_pem_file(
        PathBuf::from("../wotlk-configs")
            .join("certificate.pem"),
        PathBuf::from("../wotlk-configs")
            .join("private.pem"),
    )
    .await
    .unwrap();

    let cors = CorsLayer::new()
        .allow_origin("http://127.0.0.1:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE]);

    let app = Router::new()
        .route("/", get(check_login).post(jsonfn))
        .route("/create", get(get_id))
        .route("/download", get(download))
        .layer(cors);

    // run https server
    let addr = SocketAddr::from(([10, 0, 1, 243], ports.https));
    tracing::debug!("listening on {}", addr);
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn redirect_http_to_https(ports: Ports) {
    fn make_https(host: &str, uri: Uri, https_port: u16) -> Result<Uri, BoxError> {
        let mut parts = uri.into_parts();

        parts.scheme = Some(axum::http::uri::Scheme::HTTPS);

        if parts.path_and_query.is_none() {
            parts.path_and_query = Some("/".parse().unwrap());
        }

        let authority: Authority = host.parse()?;
        let bare_host = match authority.port() {
            Some(port_struct) => authority
                .as_str()
                .strip_suffix(port_struct.as_str())
                .unwrap()
                .strip_suffix(':')
                .unwrap(), // if authority.port() is Some(port) then we can be sure authority ends with :{port}
            None => authority.as_str(),
        };

        parts.authority = Some(format!("{bare_host}:{https_port}").parse()?);

        Ok(Uri::from_parts(parts)?)
    }

    let redirect = move |Host(host): Host, uri: Uri| async move {
        match make_https(&host, uri, ports.https) {
            Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
            Err(error) => {
                tracing::warn!(%error, "failed to convert URI to HTTPS");
                Err(StatusCode::BAD_REQUEST)
            }
        }
    };

    let addr = SocketAddr::from(([127, 0, 0, 1], ports.http));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, redirect.into_make_service())
        .await
        .unwrap();
}

async fn setup_datapool() -> Pool<MySql> {
    let file = fs::read("../wotlk-configs/wotlk-config.txt").await.unwrap();
    let res: Configs = serde_json::from_slice(&file).unwrap();

    let user = res.db_user;
    let password = res.db_password;
    MySqlPool::connect(&format!("mysql://{user}:{password}@localhost/acore_auth")).await.unwrap()
}

#[derive(sqlx::FromRow)]
pub struct AuthInfo {
    username: String,
    verifier: Vec<u8>,
    salt: Vec<u8>
}

async fn check_login() {
    let pool = setup_datapool().await;
    let user = "$Enanimate";
    let pass = "$Kbo530wy7!";

    let result = sqlx::query_as::<_, AuthInfo>(
        "SELECT username,verifier,salt FROM account WHERE username=?")
        .bind(user)
        .fetch_all(&pool)
        .await
        .unwrap();

    println!("{:?}\n{:?}\n{:?}", result[0].username, result[0].verifier, result[0].salt);
    create_verifier(user, pass, result[0].salt.clone()).await;
}

async fn create_verifier(username: &str, password: &str, salt: Vec<u8>) {
    let mut initial_hash = Sha256::new();
    initial_hash.update(format!("{}|{}", username, password).as_bytes());
    let inital = initial_hash.finalize();

    let mut final_hash = Sha256::new();
    final_hash.update(inital);
    final_hash.update(&salt);
    
    println!("\n\n{:#?}", final_hash.finalize());
}

pub struct AccountInfo {
    id: i32,
    username: String,
    salt: Vec<u8>,
    verifier: Vec<u8>,
    session_key: Vec<u8>,
    totp_secret: Vec<u8>,
    email: String,
    reg_mail: String,
    joindate: chrono::DateTime<Local>,
    last_ip: IpAddr,
    last_attempt_ip: IpAddr,
    failed_logins: i32,
    locked: i8,
    lock_country: String,
    last_login: chrono::DateTime<Local>,
    online: i32,
    expansion: i8,
    mutetime: i64,
    mutereason: String,
    muteby: String,
    locale: i8,
    os: String,
    recruiter: i32,
    totaltime: i32
}

async fn create_account() {
    let pool = setup_datapool().await;
    let user = "Enanimate";
    let pass = "Kbo530wy7!";

    let result = sqlx::query(
        "insert into account (
            id,
            username,
            salt,
            verifier,
            session_key,
            totp_secret,
            email,
            reg_mail,
            joindate,
            last_ip,
            last_attempt_ip,
            failed_logins,
            locked,
            lock_country,
            last_login,
            online,
            expansion,
            mutetime,
            mutereason,
            muteby,
            locale,
            os,
            recruiter,
            totaltime) 
        values (?)")
        //.bind(&form_output.id)
        .execute(&pool).await;
}

async fn get_id() {
    let pool = setup_datapool().await;
    let result = sqlx::query(
        "SELECT id FROM account")
        .fetch_all(&pool)
        .await
        .unwrap();

    let data: u32 = result[0].get(0);
    println!("{}", data);
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
    println!("Download Requested...");

    let file = match fs::File::open("../wotlk-client-file/wotlk-client.zip").await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };
    let len = file.metadata().await.unwrap().len();
    
    let stream = ReaderStream::new(file);
    let body = axum::body::Body::from_stream(stream);

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE, 
        "text/plain; charset=utf-8".parse().unwrap(),
    );
    headers.insert(
        header::CONTENT_DISPOSITION, 
        "attachment; filename=\"wotlk.zip\"".parse().unwrap(),
    );
    headers.insert(
        header::CONTENT_LENGTH, 
        HeaderValue::from(len),
    );

    Ok((headers, body))
}