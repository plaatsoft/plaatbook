/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlite::FromRow;
use uuid::Uuid;

use super::User;

#[derive(Clone, Serialize, FromRow)]
pub struct Post {
    pub id: Uuid,
    #[serde(skip)]
    pub user_id: Uuid,
    pub text: String,
    pub likes: i64,
    pub dislikes: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[sqlite(skip)]
    pub user: Option<User>,
    #[sqlite(skip)]
    pub auth_user_liked: Option<bool>,
    #[sqlite(skip)]
    pub auth_user_disliked: Option<bool>,
}

impl Default for Post {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::now_v7(),
            user_id: Uuid::now_v7(),
            text: String::new(),
            likes: 0,
            dislikes: 0,
            created_at: now,
            updated_at: now,
            user: None,
            auth_user_liked: None,
            auth_user_disliked: None,
        }
    }
}
