mod db;
mod error;
mod handlers;
mod models;

use axum::{Router, routing::get};
use tokio::net::TcpListener;

use crate::handlers::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let pool = db::init_pool("sqlite://buffer.db").await?;
    tracing::info!("Database connection and initialized");

    let state = AppState { pool };

    let app = Router::new()
        .route(
            "/api/v1/clips",
            get(handlers::list_clips).post(handlers::create_clip),
        )
        .route("/api/v1/clips/{id}", get(handlers::get_clip_by_id))
        .with_state(state);

    let addr = "0.0.0.0:3000";
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Server is running on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
