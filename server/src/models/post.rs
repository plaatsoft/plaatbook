/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlite::{FromRow, FromValue};
use uuid::Uuid;

use super::{PostInteractionType, User};
use crate::Context;

#[derive(Clone, Serialize, FromRow)]
pub struct Post {
    pub id: Uuid,
    pub r#type: PostType,
    #[serde(skip)]
    pub parent_post_id: Option<Uuid>,
    #[serde(skip)]
    pub user_id: Uuid,
    pub text: String,
    #[sqlite(rename = "replies")]
    pub replies_count: i64,
    #[sqlite(rename = "reposts")]
    pub reposts_count: i64,
    #[sqlite(rename = "likes")]
    pub likes_count: i64,
    #[sqlite(rename = "dislikes")]
    pub dislikes_count: i64,
    #[sqlite(rename = "views")]
    pub views_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[sqlite(skip)]
    pub parent_post: Option<Box<Post>>,
    #[sqlite(skip)]
    pub user: Option<User>,
    #[sqlite(skip)]
    pub replies: Option<Vec<Post>>,
    #[sqlite(skip)]
    pub auth_user_liked: Option<bool>,
    #[sqlite(skip)]
    pub auth_user_disliked: Option<bool>,
}

#[derive(Clone, Copy, Serialize, FromValue, Eq, PartialEq)]
pub enum PostType {
    #[serde(rename = "normal")]
    Normal = 0,
    #[serde(rename = "reply")]
    Reply = 1,
    #[serde(rename = "repost")]
    Repost = 2,
}

impl Default for Post {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::now_v7(),
            r#type: PostType::Normal,
            parent_post_id: None,
            user_id: Uuid::now_v7(),
            text: String::new(),
            replies_count: 0,
            reposts_count: 0,
            likes_count: 0,
            dislikes_count: 0,
            views_count: 0,
            created_at: now,
            updated_at: now,
            parent_post: None,
            user: None,
            replies: None,
            auth_user_liked: None,
            auth_user_disliked: None,
        }
    }
}

impl Post {
    pub fn content_post_id(&self) -> Uuid {
        if self.r#type == PostType::Repost {
            self.parent_post_id.expect("Should be some")
        } else {
            self.id
        }
    }

    pub fn fetch_user(&mut self, ctx: &Context) {
        self.user = ctx
            .database
            .query::<User>(
                format!("SELECT {} FROM users WHERE id = ? LIMIT 1", User::columns()),
                self.user_id,
            )
            .next();
    }

    pub fn fetch_parent_post(&mut self, ctx: &Context) {
        if let Some(parent_post_id) = self.parent_post_id {
            let mut parent_post = ctx
                .database
                .query::<Post>(
                    format!("SELECT {} FROM posts WHERE id = ? LIMIT 1", Post::columns()),
                    parent_post_id,
                )
                .next()
                .expect("Should be some");
            parent_post.fetch_user(ctx);
            if parent_post.r#type != PostType::Normal {
                parent_post.fetch_parent_post(ctx);
            }

            if self.r#type == PostType::Repost {
                self.replies_count = parent_post.replies_count;
                self.reposts_count = parent_post.reposts_count;
                self.likes_count = parent_post.likes_count;
                self.dislikes_count = parent_post.dislikes_count;
                self.views_count = parent_post.views_count;
            }
            self.parent_post = Some(Box::new(parent_post));
        }
    }

    pub fn fetch_relationships_and_update_views(&mut self, ctx: &Context) {
        self.fetch_user(ctx);
        self.fetch_parent_post(ctx);

        // Update views counter
        if self.r#type == PostType::Repost {
            let parent_post = self.parent_post.as_mut().expect("Should be some");
            parent_post.views_count += 1;
            ctx.database.execute(
                "UPDATE posts SET views = ? WHERE id = ?",
                (parent_post.views_count, parent_post.id),
            );
        } else {
            self.views_count += 1;
            ctx.database.execute(
                "UPDATE posts SET views = ? WHERE id = ?",
                (self.views_count, self.id),
            );
        }

        // Fetch auth user interactions
        if let Some(auth_user) = &ctx.auth_user {
            self.auth_user_liked = Some(ctx.database
                .query::<i64>(
                    "SELECT COUNT(id) FROM post_interactions WHERE post_id = ? AND user_id = ? AND type = ? LIMIT 1",
                    (self.content_post_id(), auth_user.id, PostInteractionType::Like),
                )
                .next()
                .expect("Should be some") > 0);

            self.auth_user_disliked = Some(ctx.database
                .query::<i64>(
                    "SELECT COUNT(id) FROM post_interactions WHERE post_id = ? AND user_id = ? AND type = ? LIMIT 1",
                    (self.content_post_id(), auth_user.id, PostInteractionType::Dislike),
                )
                .next()
                .expect("Should be some") > 0);
        }
    }
}
