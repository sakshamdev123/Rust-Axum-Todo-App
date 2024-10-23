use axum::extract::{Json, Path, State};
use axum::{
    http::{header::CONTENT_TYPE, HeaderValue, Response, StatusCode, Uri},
    response::IntoResponse,
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use tokio::fs;

mod database;
use database::*;

mod todos;
use todos::*;

#[tokio::main]
async fn main() {
    let db = database_connection()
        .await
        .expect("Failed to connect to Database");
    let routes = Router::new()
        .route("/root", get(root))
        .route("/user/:id", get(fetch_user_by_id))
        .route("/user/details", post(fetch_user_by_id_post_method))
        .route("/todos/all", get(Todo::get_all_todos))
        .route("/todo/create", post(Todo::create_new_todo))
        .route("/todo/:id/mark/completed", put(Todo::mark_todo_completed))
        .route("/todo/:id/delete", delete(Todo::delete_todo))
        .route("/*file", get(serve_static_file))
        .route("/", get(serve_static_file))
        .with_state(db);
    let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:".to_string() + &port)
        .await
        .unwrap();
    println!("Server running on localhost:{}", port);
    axum::serve(listener, routes).await.unwrap();
}

#[derive(Serialize)]
struct User<'a> {
    id: u32,
    username: &'a str,
}

#[derive(Deserialize)]
pub struct UserId {
    user_id: u32,
}

pub async fn fetch_user_by_id(Path(user_id): Path<u32>) -> impl IntoResponse {
    Json(User {
        username: "Elon Musk",
        id: user_id,
    })
}

pub async fn fetch_user_by_id_post_method(Json(user): Json<UserId>) -> impl IntoResponse {
    Json(User {
        id: user.user_id,
        username: "Elon Musk",
    })
}

pub async fn root(State(_db): State<MySqlPool>) -> impl IntoResponse {
    "Hello from Axum server"
}

async fn serve_static_file(uri: Uri) -> impl IntoResponse {
    let path = format!("./static{}", uri.path());
    let file_path = std::path::Path::new(&path);
    let file_path = if file_path.is_dir() {
        file_path.join("index.html")
    } else {
        file_path.to_path_buf()
    };
    match fs::read(&file_path).await {
        Ok(contents) => Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, HeaderValue::from_static(get_mime_type(&file_path)))
            .body(contents.into())
            .unwrap(),
        Err(_) => (StatusCode::NOT_FOUND, "File not found").into_response(),
    }
}

fn get_mime_type(file_path: &std::path::Path) -> &'static str {
    match file_path.extension().and_then(|ext| ext.to_str()) {
        Some("html") => "text/html",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        _ => "application/octet-stream",
    }
}
