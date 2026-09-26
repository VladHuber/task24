use crate::{
    error::AppError,
    models::{CreateTask, Task, TaskWithUser, UpdateTask},
};
use sqlx::{PgPool, query, query_as};

pub async fn create_task_with_audit(pool: &PgPool, payload: CreateTask, user_id: i64) -> Result<Task, AppError> {
    let mut tx = pool.begin().await?;

    let task_result = query_as!(
        Task,
        r#"
        INSERT INTO tasks(title, description, user_id)
        VALUES ($1, $2, $3)
        RETURNING id, title, description, completed, created_at, user_id
        "#,
        payload.title,
        payload.description,
        user_id
    )
    .fetch_one(&mut *tx)
    .await?;

    query!(
        r#"
        INSERT INTO audit_log(task_id, action)
        VALUES ($1, $2)
        "#,
        task_result.id,
        "CREATE"
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(task_result)
}

pub async fn get_tasks(
    pool: &PgPool,
    per_page: i64,
    offset: i64,
) -> Result<Vec<TaskWithUser>, AppError> {
    let tasks = query_as!(
        TaskWithUser,
        "SELECT tasks.id, title, description, completed, tasks.created_at, user_id, users.name as user_name FROM tasks 
        LEFT JOIN users ON users.id = tasks.user_id 
        ORDER BY tasks.created_at DESC
         LIMIT $1 OFFSET $2", per_page, offset
    )
    .fetch_all(pool)
    .await?;
    Ok(tasks)
}
pub async fn get_task(pool: &PgPool, task_id: i64) -> Result<Option<TaskWithUser>, AppError> {
    let task = query_as!(
        TaskWithUser,
        "SELECT tasks.id, title, description, completed, tasks.created_at, user_id, users.name as user_name FROM tasks 
        LEFT JOIN users ON users.id = tasks.user_id
        WHERE tasks.id = $1",
        task_id
    )
    .fetch_optional(pool)
    .await?;
    Ok(task)
}

pub async fn update_task(
    pool: &PgPool,
    task_id: i64,
    payload: UpdateTask,
) -> Result<Option<Task>, AppError> {
    let task = query_as!(
        Task,
        r#"UPDATE tasks 
    SET title=COALESCE($1,title),
        description=COALESCE($2,description), 
        completed=COALESCE($3,completed) 
    WHERE id=$4
    RETURNING id, title, description, completed, created_at, user_id
    "#,
        payload.title,
        payload.description,
        payload.completed,
        task_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(task)
}

pub async fn delete_task(pool: &PgPool, task_id: i64) -> Result<Option<i64>, AppError> {
    let result = query!(
        "DELETE FROM tasks 
        WHERE id = $1 RETURNING id",
        task_id
    )
    .fetch_optional(pool)
    .await?;
    Ok(result.map(|row| row.id))
}
