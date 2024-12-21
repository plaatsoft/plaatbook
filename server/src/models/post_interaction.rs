/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use chrono::{DateTime, Utc};
use sqlite::{FromRow, FromValue};
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

#[derive(FromValue)]
pub enum PostInteractionType {
    Like = 0,
    Dislike = 1,
}
