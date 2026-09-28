use axum::{
    extract::{FromRequestParts, Request, State}, http::header::AUTHORIZATION, middleware::Next, response::Response,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use crate::{
    error::AppError, AppState
};
use std::env;

#[derive(Clone, Debug)]
pub struct AuthUser {
    pub user_id: i64,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub exp: i64,
}

impl <S> FromRequestParts<S> for AuthUser where 
    S: Send + Sync
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection>
    {
        parts.extensions.get::<AuthUser>()
        .cloned()
        .ok_or(AppError::Unauthorized)
    }    
}

pub async fn auth_middleware(
    State(_state) : State<AppState>,
    mut request: Request,
    next: Next
)-> Result<Response, AppError>{
    let header = request
    .headers().get(AUTHORIZATION).ok_or(AppError::Unauthorized)?;

    let header = header.to_str().map_err(|_| AppError::Unauthorized)?;

    let token = header.strip_prefix("Bearer ").ok_or(AppError::Unauthorized)?;
     let jwt_secret = env::var("JWT_SECRET")?;

    let token_data = decode::<Claims>(
        token, 
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::new(Algorithm::HS256))?;

    let auth_user = AuthUser{
        user_id: token_data.claims.sub,
    };

    request.extensions_mut().insert(auth_user);

    Ok(next.run(request).await)

}