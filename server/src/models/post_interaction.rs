/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use sqlite::{FromRow, FromValue};
use time::DateTime;
use uuid::Uuid;

#[derive(FromRow)]
pub struct PostInteraction {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub r#type: PostInteractionType,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(FromValue)]
pub enum PostInteractionType {
    Like = 0,
    Dislike = 1,
}
