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
    pub r#type: PostType,
    #[serde(skip)]
    pub parent_post_id: Option<Uuid>,
    #[serde(skip)]
    pub user_id: Uuid,
    pub text: String,
    pub replies: i64,
    pub reposts: i64,
    pub likes: i64,
    pub dislikes: i64,
    pub views: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[sqlite(skip)]
    pub parent_post: Option<Box<Post>>,
    #[sqlite(skip)]
    pub user: Option<User>,
    #[sqlite(skip)]
    pub auth_user_reposted: Option<bool>,
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
            r#type: PostType::Normal,
            parent_post_id: None,
            user_id: Uuid::now_v7(),
            text: String::new(),
            replies: 0,
            reposts: 0,
            likes: 0,
            dislikes: 0,
            views: 0,
            created_at: now,
            updated_at: now,
            parent_post: None,
            user: None,
            auth_user_reposted: None,
            auth_user_liked: None,
            auth_user_disliked: None,
        }
    }
}

impl Post {
    pub fn fetch_user(&mut self, ctx: &Context) {
        self.user = ctx
            .database
            .query::<User>(
                format!("SELECT {} FROM users WHERE id = ? LIMIT 1", User::columns()),
                self.user_id,
            )
            .next();
    }

    pub fn fetch_relationships_and_update_views(&mut self, ctx: &Context) {
        self.fetch_user(ctx);

        // Fetch parent post
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

            // Update parent post views counter
            parent_post.views += 1;
            ctx.database.execute(
                "UPDATE posts SET views = ? WHERE id = ?",
                (parent_post.views, parent_post.id),
            );

            self.replies = parent_post.replies;
            self.reposts = parent_post.reposts;
            self.likes = parent_post.likes;
            self.dislikes = parent_post.dislikes;
            self.views = parent_post.views;
            self.parent_post = Some(Box::new(parent_post));
        } else {
            // Update views counter
            self.views += 1;
            ctx.database.execute(
                "UPDATE posts SET views = ? WHERE id = ?",
                (self.views, self.id),
            );
        }

        // Fetch auth user interactions
        if let Some(auth_user) = &ctx.auth_user {
            self.auth_user_reposted = Some(self.r#type == PostType::Repost ||
                ctx.database
                    .query::<i64>(
                        "SELECT COUNT(id) FROM posts WHERE parent_post_id = ? AND user_id = ? AND type = ? LIMIT 1",
                        (self.id, auth_user.id, PostType::Repost),
                    )
                    .next()
                    .expect("Should be some") > 0);

            self.auth_user_liked = Some(ctx.database
                .query::<i64>(
                    "SELECT COUNT(id) FROM post_interactions WHERE (post_id = ? OR post_id = ?) AND user_id = ? AND type = ? LIMIT 1",
                    (self.id, self.parent_post_id, auth_user.id, PostInteractionType::Like),
                )
                .next()
                .expect("Should be some") > 0);

            self.auth_user_disliked = Some(ctx.database
                .query::<i64>(
                    "SELECT COUNT(id) FROM post_interactions WHERE (post_id = ? OR post_id = ?) AND user_id = ? AND type = ? LIMIT 1",
                    (self.id, self.parent_post_id, auth_user.id, PostInteractionType::Dislike),
                )
                .next()
                .expect("Should be some") > 0);
        }
    }
}

// MARK: Post type
#[derive(Clone, Copy, Serialize, Eq, PartialEq)]
pub enum PostType {
    #[serde(rename = "normal")]
    Normal = 0,
    #[serde(rename = "reply")]
    Reply = 1,
    #[serde(rename = "repost")]
    Repost = 2,
}
impl From<PostType> for sqlite::Value {
    fn from(value: PostType) -> Self {
        sqlite::Value::Integer(value as i64)
    }
}
impl TryFrom<sqlite::Value> for PostType {
    type Error = sqlite::ValueError;
    fn try_from(value: sqlite::Value) -> Result<Self, Self::Error> {
        match value {
            sqlite::Value::Integer(0) => Ok(PostType::Normal),
            sqlite::Value::Integer(1) => Ok(PostType::Reply),
            sqlite::Value::Integer(2) => Ok(PostType::Repost),
            _ => Err(sqlite::ValueError),
        }
    }
}
