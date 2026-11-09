mod auth;
mod db;
mod error;
mod handlers;
mod models;

use axum::{Router, middleware, routing::get};
use tokio::net::TcpListener;

use crate::handlers::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://buffer.db".to_string());
    let api_key = std::env::var("API_KEY")?;
    let pool = db::init_pool(&db_url).await?;
    tracing::info!("Database connection and initialized");

    let state = AppState { pool, api_key };
    let app = create_app(state);

    let addr = "0.0.0.0:3000";
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Server is running on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route(
            "/api/v1/clips",
            get(handlers::list_clips).post(handlers::create_clip),
        )
        .route("/api/v1/clips/latest", get(handlers::get_latest_clip))
        .route(
            "/api/v1/clips/{id}",
            get(handlers::get_clip_by_id).delete(handlers::delete_clip_by_id),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth::auth_middleware,
        ))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    async fn setup_test_app() -> Router {
        let pool = db::init_pool("sqlite::memory:").await.unwrap();
        let state = AppState {
            pool,
            api_key: "test_secret_api".to_string(),
        };

        create_app(state)
    }

    #[tokio::test]
    async fn test_authorized() {
        let app = setup_test_app().await;
        let post_request = Request::builder()
            .method("POST")
            .uri("/api/v1/clips")
            .header("content-type", "application/json")
            .header("x-api-key", "test_secret_api")
            .body(Body::from(r#"{"content": "test"}"#))
            .unwrap();
        let post_response = app.clone().oneshot(post_request).await.unwrap();
        assert_eq!(post_response.status(), StatusCode::CREATED);

        let get_request = Request::builder()
            .method("GET")
            .uri("/api/v1/clips/latest")
            .header("content-type", "application/json")
            .header("x-api-key", "test_secret_api")
            .body(Body::empty())
            .unwrap();
        let get_response = app.clone().oneshot(get_request).await.unwrap();
        assert_eq!(get_response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_unauthorized() {
        let app = setup_test_app().await;
        let post_request = Request::builder()
            .method("POST")
            .uri("/api/v1/clips")
            .header("content-type", "application/json")
            .header("x-api-key", "wrong_test_secret_api")
            .body(Body::from(r#"{"content": "test"}"#))
            .unwrap();
        let post_response = app.clone().oneshot(post_request).await.unwrap();
        assert_eq!(post_response.status(), StatusCode::UNAUTHORIZED);

        let get_request = Request::builder()
            .method("GET")
            .uri("/api/v1/clips/latest")
            .header("content-type", "application/json")
            .header("x-api-key", "wrong_test_secret_api")
            .body(Body::empty())
            .unwrap();
        let get_response = app.clone().oneshot(get_request).await.unwrap();
        assert_eq!(get_response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_empty_clip() {
        let app = setup_test_app().await;
        let post_request = Request::builder()
            .method("POST")
            .uri("/api/v1/clips")
            .header("content-type", "application/json")
            .header("x-api-key", "test_secret_api")
            .body(Body::from(r#"{"content": "  "}"#))
            .unwrap();
        let post_response = app.clone().oneshot(post_request).await.unwrap();
        assert_eq!(post_response.status(), StatusCode::BAD_REQUEST);
    }
}
