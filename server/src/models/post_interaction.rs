/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use bsqlite::{FromRow, FromValue};
use chrono::{DateTime, Utc};
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

impl Default for PostInteraction {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::now_v7(),
            post_id: Uuid::nil(),
            user_id: Uuid::nil(),
            r#type: PostInteractionType::Like,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(FromValue)]
pub enum PostInteractionType {
    Like = 0,
    Dislike = 1,
}
