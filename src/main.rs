use std::env;
use sqlx::{PgPool,postgres::PgPoolOptions};

use axum::{routing::{get,post,patch,delete}, Router, middleware as axum_middleware};

use crate::handlers::tasks::{*};
use crate::middleware::auth::auth_middleware;

mod handlers;
mod models;
mod repository;
mod middleware;
mod error;

#[derive(Clone)]
pub struct AppState{
    db: PgPool,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Не найдено");
    
    let pool = PgPoolOptions::new().max_connections(5).connect(&database_url).await?;
    let state = AppState{
        db: pool
    };
    let protected_routes = Router::new()
    .route("/tasks", post(create_task))
    .route("/tasks", get(get_tasks))
    .route("/tasks/{id}", get(get_task))
    .route("/tasks/{id}", delete(delete_task))
    .route("/tasks/{id}", patch(update_task));

    let protected_routes = protected_routes
    .route_layer(
        axum_middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        )
    );
    let app = Router::new()
    .merge(protected_routes)
    .route("/auth/register", post(handlers::auth::register))
    .route("/auth/login", post(handlers::auth::login))
    .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    axum::serve(listener, app).await?;
    Ok(())
}
