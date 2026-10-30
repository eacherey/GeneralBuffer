use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Clone, FromRow)]
pub struct Clip {
    pub id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateClip {
    pub content: String,
}
