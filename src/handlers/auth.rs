use axum::{
    Json,
    extract::{State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use argon2::{
    Argon2, PasswordHash, PasswordVerifier, password_hash::PasswordHasher
};
use jsonwebtoken::{encode, Header, EncodingKey};
use std::env;

use crate::{AppState, repository::{self, auth::{AutorizationUser, CreateUser, User, get_user}}, middleware::auth::Claims};
use crate::error::AppError;

#[derive(Deserialize)]
pub struct RegisterParams{
    email: String,
    password: String,
    name: String
}

pub async fn register(
    State(state) : State<AppState>,
    Json(payload): Json<RegisterParams>
) -> Result<(StatusCode, Json<User>), AppError>{
    
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(payload.password.as_bytes())?;
    let hash_string = password_hash.to_string();
    let user = repository::auth::create_user(
        &state.db, CreateUser{password_hash: hash_string, email: payload.email, name: payload.name}
    ).await?;
    Ok((StatusCode::CREATED, Json(user)))
}
#[derive(Deserialize)]
pub struct LoginRequest{
    email: String,
    password: String
}
#[derive(Serialize)]
pub struct AuthResonse{
    token: String
}


pub async fn login(
    State(state) : State<AppState>,
    Json(payload): Json<LoginRequest>
)-> Result<Json<AuthResonse>, AppError>{
    let user = get_user(&state.db, 
        AutorizationUser{email: payload.email}).await?.ok_or(AppError::Unauthorized)?;    
    let parsed_hash =   PasswordHash::new(&user.password_hash)?;
    if Argon2::default().verify_password(payload.password.as_bytes(), &parsed_hash).is_err(){
        return Err(AppError::Unauthorized);
    }

    let exp = chrono::Utc::now()
    .checked_add_signed(chrono::Duration::hours(24))
    .expect("valid timestamp")
    .timestamp();
    
    let claims = Claims{
        sub: user.id,
        exp: exp
    };
    let jwt_secret = env::var("JWT_SECRET")?;
    let token = encode(&Header::default(),&claims,&EncodingKey::from_secret(jwt_secret.as_bytes()))?;

    Ok(Json(AuthResonse { token }))
}