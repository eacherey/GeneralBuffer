use chrono::Utc;
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use std::str::FromStr;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{Clip, CreateClip};

pub async fn init_pool(db_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let option = SqliteConnectOptions::from_str(db_url)?.create_if_missing(true);
    let pool = SqlitePool::connect_with(option).await?;

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS clips (
        id TEXT PRIMARY KEY NOT NULL,
        content TEXT NOT NULL,
        created_at TEXT NOT NULL
        );
        ",
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}

pub async fn insert_clip(pool: &SqlitePool, clip: CreateClip) -> Result<Clip, AppError> {
    let id = Uuid::new_v4();
    let created_at = Utc::now();

    sqlx::query("INSERT INTO clips (id, content, created_at) VALUES ($1, $2, $3)")
        .bind(id)
        .bind(&clip.content)
        .bind(created_at)
        .execute(pool)
        .await?;

    Ok(Clip {
        id,
        content: clip.content,
        created_at,
    })
}

pub async fn get_clip(pool: &SqlitePool) -> Result<Vec<Clip>, AppError> {
    let clips = sqlx::query_as::<_, Clip>(
        "SELECT id, content, created_at FROM clips ORDER BY created_at DESC",
    )
    .fetch_all(pool)
    .await?;

    Ok(clips)
}

pub async fn get_clip_by_id(pool: &SqlitePool, id: Uuid) -> Result<Option<Clip>, AppError> {
    let clip = sqlx::query_as::<_, Clip>("SELECT id, content, created_at FROM clips WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?;

    Ok(clip)
}
