/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use chrono::{DateTime, Utc};
use sqlite::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct PostInteraction {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub r#type: PostInteractionType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub enum PostInteractionType {
    Like = 0,
    Dislike = 1,
}
impl From<PostInteractionType> for sqlite::Value {
    fn from(value: PostInteractionType) -> Self {
        sqlite::Value::Integer(value as i64)
    }
}
impl TryFrom<sqlite::Value> for PostInteractionType {
    type Error = sqlite::ValueError;
    fn try_from(value: sqlite::Value) -> Result<Self, Self::Error> {
        match value {
            sqlite::Value::Integer(0) => Ok(PostInteractionType::Like),
            sqlite::Value::Integer(1) => Ok(PostInteractionType::Dislike),
            _ => Err(sqlite::ValueError),
        }
    }
}
