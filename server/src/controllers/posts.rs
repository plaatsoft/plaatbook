/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use chrono::Utc;
use http::{Request, Response, Status};
use router::Path;
use serde::Deserialize;
use uuid::Uuid;
use validate::Validate;

use crate::controllers::not_found;
use crate::models::{Post, PostInteraction, PostInteractionType, User, UserRole};
use crate::Context;

// MARK: Helpers
fn find_post(ctx: &Context, path: &Path) -> Option<Post> {
    let post_id = match path.get("post_id").expect("Should exists").parse::<Uuid>() {
        Ok(id) => id,
        Err(_) => return None,
    };

    ctx.database
        .query::<Post>(
            format!("SELECT {} FROM posts WHERE id = ? LIMIT 1", Post::columns()),
            post_id,
        )
        .next()
}

fn fetch_posts_relationships(post: Post, ctx: &Context) -> Post {
    let mut post = post;
    post.user = ctx
        .database
        .query::<User>(
            format!("SELECT {} FROM users WHERE id = ? LIMIT 1", User::columns()),
            post.user_id,
        )
        .next();

    if let Some(auth_user) = &ctx.auth_user {
        post.auth_user_liked = Some(
            ctx.database
                .query::<i64>(
                    "SELECT COUNT(id) FROM post_interactions WHERE post_id = ? AND user_id = ? AND type = ? LIMIT 1",
                    (post.id, auth_user.id, PostInteractionType::Like),
                )
                .next()
                .expect("Should be some") > 0);

        post.auth_user_disliked = Some(
            ctx.database
                .query::<i64>(
                    "SELECT COUNT(id) FROM post_interactions WHERE post_id = ? AND user_id = ? AND type = ? LIMIT 1",
                    (post.id, auth_user.id, PostInteractionType::Dislike),
                )
                .next()
                .expect("Should be some") > 0);
    }
    post
}

fn remove_post_like(database: &sqlite::Connection, post: &Post, auth_user: &User) {
    // Remove post like interaction
    database.execute(
        "DELETE FROM post_interactions WHERE post_id = ? AND user_id = ? AND type = ?",
        (post.id, auth_user.id, PostInteractionType::Like),
    );
    if database.affected_rows() > 0 {
        database.execute(
            "UPDATE posts SET likes = ? WHERE id = ?",
            (post.likes - 1, post.id),
        );
    }
}

fn remove_post_dislike(database: &sqlite::Connection, post: &Post, auth_user: &User) {
    // Remove post dislike interaction
    database.execute(
        "DELETE FROM post_interactions WHERE post_id = ? AND user_id = ? AND type = ?",
        (post.id, auth_user.id, PostInteractionType::Dislike),
    );
    if database.affected_rows() > 0 {
        database.execute(
            "UPDATE posts SET dislikes = ? WHERE id = ?",
            (post.dislikes - 1, post.id),
        );
    }
}

// MARK: Posts index
pub fn posts_index(_: &Request, ctx: &Context, _: &Path) -> Response {
    // Authorization
    // -

    let posts = ctx
        .database
        .query::<Post>(
            format!(
                "SELECT {} FROM posts ORDER BY created_at DESC",
                Post::columns()
            ),
            (),
        )
        .map(|post| fetch_posts_relationships(post, ctx))
        .collect::<Vec<_>>();

    Response::new().json(posts)
}

// MARK: Posts create
pub fn posts_create(req: &Request, ctx: &Context, _: &Path) -> Response {
    // Authorization
    let auth_user = match ctx.auth_user.as_ref() {
        Some(user) => user,
        None => {
            return Response::new()
                .status(Status::Unauthorized)
                .body("401 Unauthorized")
        }
    };

    // Parse and validate body
    #[derive(Deserialize, Validate)]
    struct Body {
        #[validate(length(min = 1))]
        text: String,
    }
    let body = match serde_urlencoded::from_str::<Body>(&req.body) {
        Ok(body) => body,
        Err(_) => {
            return Response::new()
                .status(Status::BadRequest)
                .body("400 Bad Request");
        }
    };
    if let Err(errors) = body.validate() {
        return Response::new().status(Status::BadRequest).json(errors);
    }

    // Create a new post
    let post = Post {
        user_id: auth_user.id,
        text: body.text,
        user: Some(auth_user.clone()),
        ..Default::default()
    };
    ctx.database.execute(
        format!(
            "INSERT INTO posts ({}) VALUES ({})",
            Post::columns(),
            Post::values()
        ),
        post.clone(),
    );

    Response::new().json(post)
}

// MARK: Posts show
pub fn posts_show(req: &Request, ctx: &Context, path: &Path) -> Response {
    let post = match find_post(ctx, path) {
        Some(post) => post,
        None => return not_found(req, ctx, path),
    };

    // Authorization
    // -

    Response::new().json(fetch_posts_relationships(post, ctx))
}

// MARK: Posts update
pub fn posts_update(req: &Request, ctx: &Context, path: &Path) -> Response {
    let mut post = match find_post(ctx, path) {
        Some(post) => post,
        None => return not_found(req, ctx, path),
    };

    // Authorization
    let auth_user = ctx.auth_user.as_ref().expect("Not authed");
    if !(post.user_id == auth_user.id || auth_user.role == UserRole::Admin) {
        return Response::new()
            .status(Status::Unauthorized)
            .body("401 Unauthorized");
    }

    // Parse and validate body
    #[derive(Deserialize, Validate)]
    struct Body {
        #[validate(length(min = 1))]
        text: String,
    }
    let body = match serde_urlencoded::from_str::<Body>(&req.body) {
        Ok(body) => body,
        Err(_) => {
            return Response::new()
                .status(Status::BadRequest)
                .body("400 Bad Request");
        }
    };
    if let Err(errors) = body.validate() {
        return Response::new().status(Status::BadRequest).json(errors);
    }

    // Update post
    post.text = body.text;
    post.updated_at = Utc::now();
    ctx.database.execute(
        "UPDATE posts SET text = ?, updated_at = ? WHERE id = ?",
        (post.text.clone(), post.updated_at, post.id),
    );

    Response::new().json(post)
}

// MARK: Posts like
pub fn posts_like(req: &Request, ctx: &Context, path: &Path) -> Response {
    let post = match find_post(ctx, path) {
        Some(post) => post,
        None => return not_found(req, ctx, path),
    };

    // Authorization
    let auth_user = match ctx.auth_user.as_ref() {
        Some(user) => user,
        None => {
            return Response::new()
                .status(Status::Unauthorized)
                .body("401 Unauthorized")
        }
    };

    // Remove possible old post interaction
    remove_post_like(&ctx.database, &post, auth_user);
    remove_post_dislike(&ctx.database, &post, auth_user);

    // Create new post like interaction
    let now = Utc::now();
    let post_interaction = PostInteraction {
        id: Uuid::now_v7(),
        post_id: post.id,
        user_id: auth_user.id,
        r#type: PostInteractionType::Like,
        created_at: now,
        updated_at: now,
    };
    ctx.database.execute(
        format!(
            "INSERT INTO post_interactions ({}) VALUES ({})",
            PostInteraction::columns(),
            PostInteraction::values()
        ),
        post_interaction,
    );
    ctx.database.execute(
        "UPDATE posts SET likes = ? WHERE id = ?",
        (post.likes + 1, post.id),
    );

    Response::new()
}

// MARK: Posts like delete
pub fn posts_like_delete(req: &Request, ctx: &Context, path: &Path) -> Response {
    let post = match find_post(ctx, path) {
        Some(post) => post,
        None => return not_found(req, ctx, path),
    };

    // Authorization
    let auth_user = match ctx.auth_user.as_ref() {
        Some(user) => user,
        None => {
            return Response::new()
                .status(Status::Unauthorized)
                .body("401 Unauthorized")
        }
    };

    // Remove post like
    remove_post_like(&ctx.database, &post, auth_user);
    Response::new()
}

// MARK: Posts dislike
pub fn posts_dislike(req: &Request, ctx: &Context, path: &Path) -> Response {
    let post = match find_post(ctx, path) {
        Some(post) => post,
        None => return not_found(req, ctx, path),
    };

    // Authorization
    let auth_user = match ctx.auth_user.as_ref() {
        Some(user) => user,
        None => {
            return Response::new()
                .status(Status::Unauthorized)
                .body("401 Unauthorized")
        }
    };

    // Remove possible old post interaction
    remove_post_like(&ctx.database, &post, auth_user);
    remove_post_dislike(&ctx.database, &post, auth_user);

    // Create new post dislike interaction
    let now = Utc::now();
    let post_interaction = PostInteraction {
        id: Uuid::now_v7(),
        post_id: post.id,
        user_id: auth_user.id,
        r#type: PostInteractionType::Dislike,
        created_at: now,
        updated_at: now,
    };
    ctx.database.execute(
        format!(
            "INSERT INTO post_interactions ({}) VALUES ({})",
            PostInteraction::columns(),
            PostInteraction::values()
        ),
        post_interaction,
    );
    ctx.database.execute(
        "UPDATE posts SET dislikes = ? WHERE id = ?",
        (post.dislikes + 1, post.id),
    );

    Response::new()
}

// MARK: Posts dislike delete
pub fn posts_dislike_delete(req: &Request, ctx: &Context, path: &Path) -> Response {
    let post = match find_post(ctx, path) {
        Some(post) => post,
        None => return not_found(req, ctx, path),
    };

    // Authorization
    let auth_user = match ctx.auth_user.as_ref() {
        Some(user) => user,
        None => {
            return Response::new()
                .status(Status::Unauthorized)
                .body("401 Unauthorized")
        }
    };

    // Remove post dislike
    remove_post_dislike(&ctx.database, &post, auth_user);
    Response::new()
}

// MARK: Posts delete
pub fn posts_delete(req: &Request, ctx: &Context, path: &Path) -> Response {
    let post = match find_post(ctx, path) {
        Some(post) => post,
        None => return not_found(req, ctx, path),
    };

    // Authorization
    let auth_post = ctx.auth_user.as_ref().expect("Not authed");
    if !(post.user_id == auth_post.id || auth_post.role == UserRole::Admin) {
        return Response::new()
            .status(Status::Unauthorized)
            .body("401 Unauthorized");
    }

    // Delete post
    ctx.database
        .execute("DELETE FROM posts WHERE id = ?", post.id);

    Response::new()
}
