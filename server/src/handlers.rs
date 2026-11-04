use axum::extract::Path;
use axum::{Json, extract::State, http::StatusCode};
use common::{Clip, CreateClip};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::db;
use crate::error::AppError;

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub api_key: String,
}

pub async fn create_clip(
    State(state): State<AppState>,
    Json(payload): Json<CreateClip>,
) -> Result<(StatusCode, Json<Clip>), AppError> {
    if payload.content.trim().is_empty() {
        return Err(AppError::Validation("Clip cannot be empty".to_string()));
    }

    let clip = db::insert_clip(&state.pool, payload).await?;

    Ok((StatusCode::CREATED, Json(Clip::from(clip))))
}

pub async fn list_clips(State(state): State<AppState>) -> Result<Json<Vec<Clip>>, AppError> {
    let clips = db::get_clip(&state.pool).await?;

    let clips = clips.into_iter().map(Clip::from).collect();

    Ok(Json(clips))
}

pub async fn get_clip_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Clip>, AppError> {
    let clip = db::get_clip_by_id(&state.pool, id).await?;

    match clip {
        Some(clip) => Ok(Json(Clip::from(clip))),
        None => Err(AppError::NotFound),
    }
}

pub async fn get_latest_clip(State(state): State<AppState>) -> Result<Json<Clip>, AppError> {
    let clip = db::get_latest_clip(&state.pool).await?;

    match clip {
        Some(clip) => Ok(Json(Clip::from(clip))),
        None => Err(AppError::NotFound),
    }
}

pub async fn delete_clip_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Clip>, AppError> {
    let clip = db::delete_clip_by_id(&state.pool, id).await?;

    match clip {
        Some(clip) => Ok(Json(Clip::from(clip))),
        None => Err(AppError::NotFound),
    }
}
