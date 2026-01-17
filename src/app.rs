// External USEs
use axum::{Router, Extension, routing::get};
use tokio::net::TcpListener;
use sqlx::PgPool;

// Internal USEs
use crate::conn::create_pool;
use std::env;

// Routers
use crate::routes::{
    characters
};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool
} 

async fn create_app() -> Router {
    let pool: PgPool = create_pool().await.expect("Failed to create pool");
    let state: AppState = AppState { pool: pool };
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/characters", characters::router()) 
        .layer(Extension(state))
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let app: Router = create_app().await;
    let port: String = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("127.0.0.1:{}", port);

    let listener = TcpListener::bind(addr)
        .await?;
    axum::serve(listener, app)
        .await?;
    Ok(())    
}
