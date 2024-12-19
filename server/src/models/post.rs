/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlite::FromRow;
use uuid::Uuid;

use super::{PostInteractionType, User};
use crate::Context;

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

impl Post {
    pub fn fetch_relationships(&mut self, ctx: &Context) {
        self.user = ctx
            .database
            .query::<User>(
                format!("SELECT {} FROM users WHERE id = ? LIMIT 1", User::columns()),
                self.user_id,
            )
            .next();

        if let Some(auth_user) = &ctx.auth_user {
            self.auth_user_liked = Some(
            ctx.database
                .query::<i64>(
                    "SELECT COUNT(id) FROM post_interactions WHERE post_id = ? AND user_id = ? AND type = ? LIMIT 1",
                    (self.id, auth_user.id, PostInteractionType::Like),
                )
                .next()
                .expect("Should be some") > 0);

            self.auth_user_disliked = Some(    ctx.database
                .query::<i64>(
                    "SELECT COUNT(id) FROM post_interactions WHERE post_id = ? AND user_id = ? AND type = ? LIMIT 1",
                    (self.id, auth_user.id, PostInteractionType::Dislike),
                )
                .next()
                .expect("Should be some") > 0);
        }
    }
}
