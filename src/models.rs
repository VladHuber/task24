use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Debug)]
pub struct Task{
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: chrono::NaiveDateTime,
    pub user_id: i64
}

#[derive(Deserialize)]
pub struct CreateTask{
    pub title: String,
    pub description: Option<String>
}
#[derive(Deserialize)]
pub struct UpdateTask{
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}

#[derive(FromRow, Serialize, Debug)]
pub struct TaskWithUser{
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: chrono::NaiveDateTime,
    pub user_id: i64,
    pub user_name: Option<String>
}

#[derive(Deserialize)]
pub struct PaginationParams{
    pub page: Option<i64>,
    pub per_page: Option<i64>
}