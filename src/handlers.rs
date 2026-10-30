use axum::{Json, extract::State, http::StatusCode};
use sqlx::SqlitePool;

use crate::db::{get_clip, insert_clip};
use crate::error::AppError;
use crate::models::{Clip, CreateClip};

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
}

pub async fn create_clip(
    State(state): State<AppState>,
    Json(payload): Json<CreateClip>,
) -> Result<(StatusCode, Json<Clip>), AppError> {
    if payload.content.trim().is_empty() {
        return Err(AppError::Validation("Clip cannot be empty".to_string()));
    }

    let clip = insert_clip(&state.pool, payload).await?;

    Ok((StatusCode::CREATED, Json(clip)))
}

pub async fn list_clips(State(state): State<AppState>) -> Result<Json<Vec<Clip>>, AppError> {
    let clips = get_clip(&state.pool).await?;

    Ok(Json(clips))
}
