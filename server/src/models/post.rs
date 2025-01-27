/*
 * Copyright (c) 2024-2025 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use std::sync::LazyLock;

use from_enum::FromEnum;
use regex::Regex;
use sqlite::{FromRow, FromValue};
use time::DateTime;
use uuid::Uuid;

use super::{PostInteractionType, User};
use crate::{api, Context};

// MARK: Post
#[derive(Clone, FromRow)]
pub struct Post {
    pub id: Uuid,
    pub r#type: PostType,
    pub parent_post_id: Option<Uuid>,
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
    pub created_at: DateTime,
    pub updated_at: DateTime,
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

#[derive(Clone, Copy, Eq, PartialEq, FromEnum, FromValue)]
#[from_enum(api::PostType)]
pub enum PostType {
    Normal = 0,
    Reply = 1,
    Repost = 2,
}

impl Default for Post {
    fn default() -> Self {
        let now = DateTime::now();
        Self {
            id: Uuid::now_v7(),
            r#type: PostType::Normal,
            parent_post_id: None,
            user_id: Uuid::nil(),
            text: "".to_string(),
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

impl From<Post> for api::Post {
    fn from(post: Post) -> Self {
        let text_html = render_markdown(&post.text);
        Self {
            id: post.id,
            r#type: post.r#type.into(),
            text: post.text,
            text_html,
            replies_count: post.replies_count,
            reposts_count: post.reposts_count,
            likes_count: post.likes_count,
            dislikes_count: post.dislikes_count,
            views_count: post.views_count,
            created_at: post.created_at,
            updated_at: post.updated_at,
            parent_post: post.parent_post.map(|post| Box::new((*post).into())),
            user: post.user.map(|user| user.into()),
            replies: post
                .replies
                .map(|replies| replies.into_iter().map(|post| post.into()).collect()),
            auth_user_liked: post.auth_user_liked,
            auth_user_disliked: post.auth_user_disliked,
        }
    }
}

// MARK: Relationships
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

    pub fn fetch_user_interactions(&mut self, ctx: &Context) {
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

    pub fn update_views(&mut self, ctx: &Context) {
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
    }

    pub fn fetch_relationships(&mut self, ctx: &Context) {
        self.fetch_user(ctx);
        self.fetch_parent_post(ctx);
        self.fetch_user_interactions(ctx);
        self.update_views(ctx);
    }
}

// MARK: Post Markdown
static URL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(https?://[\w./?=&-]+)").expect("Should compile"));
static BOLD_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\*\*([^\*]*)\*\*").expect("Should compile"));
static ITALIC_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\*([^\*]*)\*").expect("Should compile"));
static PARAGRAPH_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\n\n").expect("Should compile"));

fn render_markdown(text: &str) -> String {
    // Convert URLs to links
    let mut text = URL_REGEX
        .replace_all(
            text,
            r#"<a href="$1" target="_blank" rel="noreferrer">$1</a>"#,
        )
        .to_string();

    // Convert bold text
    text = BOLD_REGEX.replace_all(&text, r#"<b>$1</b>"#).to_string();

    // Convert italic text
    text = ITALIC_REGEX.replace_all(&text, r#"<i>$1</i>"#).to_string();

    // Convert paragraphs
    text = PARAGRAPH_REGEX.replace_all(&text, r#"</p><p>"#).to_string();
    format!("<p>{}</p>", text)
}

// MARK: Tests
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_render_markdown_urls() {
        let input = "Check this out: https://example.com";
        let expected = r#"<p>Check this out: <a href="https://example.com" target="_blank" rel="noreferrer">https://example.com</a></p>"#;
        assert_eq!(render_markdown(input), expected);
    }

    #[test]
    fn test_render_markdown_bold() {
        let input = "This is **bold** text.";
        let expected = r#"<p>This is <b>bold</b> text.</p>"#;
        assert_eq!(render_markdown(input), expected);
    }

    #[test]
    fn test_render_markdown_italic() {
        let input = "This is *italic* text.";
        let expected = r#"<p>This is <i>italic</i> text.</p>"#;
        assert_eq!(render_markdown(input), expected);
    }

    #[test]
    fn test_render_markdown_paragraphs() {
        let input = "First paragraph.\n\nSecond paragraph.";
        let expected = r#"<p>First paragraph.</p><p>Second paragraph.</p>"#;
        assert_eq!(render_markdown(input), expected);
    }

    #[test]
    fn test_render_markdown_combined() {
        let input = "Visit **https://example.com** for more *details*.\n\nThank you!";
        let expected = r#"<p>Visit <b><a href="https://example.com" target="_blank" rel="noreferrer">https://example.com</a></b> for more <i>details</i>.</p><p>Thank you!</p>"#;
        assert_eq!(render_markdown(input), expected);
    }
}
