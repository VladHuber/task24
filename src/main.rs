use std::env;
use sqlx::{PgPool,postgres::PgPoolOptions};

use axum::{routing::{get,post,patch,delete}, Router};

mod handlers;
mod models;
mod repository;
mod error;

#[derive(Clone)]
pub struct AppState{
    db: PgPool
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Не найдено");
    
    let pool = PgPoolOptions::new().max_connections(5).connect(&database_url).await?;
    let state = AppState{
        db: pool
    };

    let app = Router::new()
    .route("/tasks", post(handlers::tasks::create_task))
    .route("/tasks", get(handlers::tasks::get_tasks))
    .route("/tasks/{id}", get(handlers::tasks::get_task))
    .route("/tasks/{id}", patch(handlers::tasks::update_task))
    .route("/tasks/{id}", delete(handlers::tasks::delete_task))
    .route("/auth/register", post(handlers::auth::register))
    .route("/auth/login", post(handlers::auth::login))
    .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    axum::serve(listener, app).await?;
    Ok(())
}
