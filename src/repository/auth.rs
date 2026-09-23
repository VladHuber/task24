use crate::{
    error::AppError
};
use sqlx::{PgPool, query_as};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct CreateUser{
    pub name: String,
    pub email: String,
    pub password_hash: String
}

#[derive(FromRow, Serialize, Debug)]
pub struct User{
    pub id: i64,
    pub name: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>
}

pub async fn create_user(pool: &PgPool, payload: CreateUser) -> Result<User, AppError> {
    let user = query_as!(
        User,
        r#"
        INSERT INTO users(name, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id, name, email, created_at
        "#,
        payload.name,
        payload.email,
        payload.password_hash
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub struct AutorizationUser{
    pub email: String
}
pub struct UserLoginInfo{
    pub id: i64,
    pub password_hash: String
}
pub async fn get_user(pool: &PgPool, payload: AutorizationUser)-> Result<Option<UserLoginInfo>, AppError>{
    let user = query_as!(UserLoginInfo,
        "SELECT id, password_hash FROM users WHERE email=$1", &payload.email)
        .fetch_optional(pool).await?;
    Ok(user)   
}