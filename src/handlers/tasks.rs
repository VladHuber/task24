use axum::{
    Json,
    extract::{Path, State, Query, Extension},
    http::StatusCode,
};

use crate::{AppState, models::{PaginationParams, TaskWithUser}};
use crate::models::{CreateTask, Task, UpdateTask};
use crate::repository;
use crate::error::AppError;
use crate::middleware::auth::AuthUser;

pub async fn create_task(
    Extension(auth_user): Extension<AuthUser>,
    State(state): State<AppState>,
    Json(payload): Json<CreateTask>,
) -> Result<(StatusCode, Json<Task>), AppError> {
    let task = repository::tasks::create_task_with_audit(&state.db, payload, auth_user.user_id)
        .await?;

    Ok((StatusCode::CREATED, Json(task)))
}
pub async fn get_tasks(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>
) -> Result<(StatusCode, Json<Vec<TaskWithUser>>), AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(10).max(1).min(100);
    
    let offset = (page - 1) * per_page;

    let tasks = repository::tasks::get_tasks(&state.db, per_page, offset)
       .await?;

    Ok((StatusCode::OK, Json(tasks)))
}

pub async fn get_task(
    State(state): State<AppState>,
    Path(task_id): Path<i64>,
) -> Result<(StatusCode, Json<TaskWithUser>), AppError> {
    let task = repository::tasks::get_task(&state.db, task_id)
        .await?.ok_or(AppError::NotFound)?;
    Ok((StatusCode::OK, Json(task)))
}

pub async fn update_task(
    State(state): State<AppState>,
    Path(task_id): Path<i64>,
    Json(payload) : Json<UpdateTask>
) -> Result<(StatusCode, Json<Task>), AppError> {
    let task = repository::tasks::update_task(&state.db, task_id, payload).await?.ok_or(AppError::NotFound)?;
    Ok((StatusCode::OK, Json(task)))
}

pub async fn delete_task(
    State(state): State<AppState>,
    Path(task_id): Path<i64>,
) -> Result<(StatusCode, Json<i64>), AppError> {
    let task = repository::tasks::delete_task(&state.db, task_id)
        .await?.ok_or(AppError::NotFound)?;
    Ok((StatusCode::OK, Json(task)))
}