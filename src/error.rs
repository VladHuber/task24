use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use sqlx::Error as SqlxError;

pub enum AppError {
    NotFound,
    Database(SqlxError),
    HashError(String),
    Unauthorized,
    EnvNotFound,
    JwtError
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound => {
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({
                        "error": "Task not found"
                    })),
                )
                    .into_response()
            }

            AppError::Database(err) => {
                eprintln!("Database error: {err}");

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "error": "Internal server error"
                    })),
                )
                    .into_response()
            }
            AppError::HashError(err) => {
                eprintln!("Password hashing error: {err}"); // Логируем реальную ошибку в консоль

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "error": "Internal server error" // Клиенту скрываем детали безопасности
                    })),
                )
                    .into_response()
            },
            AppError::Unauthorized =>{
                 (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({
                        "error": "Unauthorized"
                    })),
                )
                    .into_response()
            },
            AppError::EnvNotFound =>{
                eprintln!("Env not found"); // Логируем реальную ошибку в консоль

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "error": "Internal server error" // Клиенту скрываем детали безопасности
                    })),
                )
                    .into_response()
            }
            AppError::JwtError =>{
                eprintln!("Jwt error"); // Логируем реальную ошибку в консоль

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "error": "Internal server error" // Клиенту скрываем детали безопасности
                    })),
                )
                    .into_response()
            }
        }
    }
}

impl From<SqlxError> for AppError {
    fn from(err: SqlxError) -> Self {
        AppError::Database(err)
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(err: argon2::password_hash::Error) -> Self{
        AppError::HashError(err.to_string())
    }
}

impl From<argon2::password_hash::phc::Error> for AppError {
    fn from(_err: argon2::password_hash::phc::Error) -> Self{
        AppError::Unauthorized
    }
}

impl From<std::env::VarError> for AppError {
    fn from(_value: std::env::VarError) -> Self {
        AppError::EnvNotFound
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(_value: jsonwebtoken::errors::Error) -> Self {
        AppError::JwtError
    }
}