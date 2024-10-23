use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, MySqlPool};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Todo {
    id: i32,
    title: String,
    description: Option<String>,
    status: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct CreateTodo {
    title: String,
    description: Option<String>,
}

impl Todo {
    pub async fn get_all_todos(State(db): State<MySqlPool>) -> impl IntoResponse {
        let res: Result<Vec<Self>, Error> =
            sqlx::query_as("SELECT id, title, description, status FROM todos")
                .fetch_all(&db)
                .await;
        match res {
            Ok(todos) => (StatusCode::OK, Json(todos)).into_response(),
            Err(_e) => (StatusCode::INTERNAL_SERVER_ERROR, _e.to_string()).into_response(),
        }
    }

    pub async fn create_new_todo(
        State(db): State<MySqlPool>,
        Json(body): Json<CreateTodo>,
    ) -> impl IntoResponse {
        let res = sqlx::query("INSERT INTO todos (title, description) values (?, ?)")
            .bind(&body.title)
            .bind(&body.description)
            .execute(&db)
            .await;
        match res {
            Ok(todo) => (
                StatusCode::CREATED,
                Json(Self {
                    id: todo.last_insert_id() as i32,
                    description: body.description.clone(),
                    status: "New".to_string(),
                    title: body.title.clone(),
                }),
            )
                .into_response(),
            Err(_e) => (StatusCode::INTERNAL_SERVER_ERROR, _e.to_string()).into_response(),
        }
    }

    pub async fn mark_todo_completed(
        State(db): State<MySqlPool>,
        Path(id): Path<i32>,
    ) -> impl IntoResponse {
        let res = sqlx::query("UPDATE todos SET status = ? WHERE id = ?")
            .bind("Completed")
            .bind(id)
            .execute(&db)
            .await;
        match res {
            Ok(_) => StatusCode::OK,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub async fn delete_todo(
        State(db): State<MySqlPool>,
        Path(id): Path<i32>,
    ) -> impl IntoResponse {
        let res = sqlx::query("DELETE FROM todos WHERE id = ?")
            .bind(id)
            .execute(&db)
            .await;
        match res {
            Ok(_) => StatusCode::OK,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
