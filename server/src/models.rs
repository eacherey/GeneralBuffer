use chrono::{DateTime, Utc};
use common::Clip;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct ClipDb {
    pub id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl From<ClipDb> for Clip {
    fn from(db: ClipDb) -> Self {
        Clip {
            id: db.id,
            content: db.content,
            created_at: db.created_at,
        }
    }
}
