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

use crate::consts::LIMIT_DEFAULT;
use crate::controllers::not_found;
use crate::models::{
    IndexQuery, Post, PostInteraction, PostInteractionType, PostType, User, UserRole,
};
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

fn remove_post_like(database: &sqlite::Connection, post_id: Uuid, auth_user: &User) {
    // Remove post like interaction
    database.execute(
        "DELETE FROM post_interactions WHERE post_id = ? AND user_id = ? AND type = ?",
        (post_id, auth_user.id, PostInteractionType::Like),
    );
    if database.affected_rows() > 0 {
        database.execute("UPDATE posts SET likes = likes - 1 WHERE id = ?", post_id);
    }
}

fn remove_post_dislike(database: &sqlite::Connection, post_id: Uuid, auth_user: &User) {
    // Remove post dislike interaction
    database.execute(
        "DELETE FROM post_interactions WHERE post_id = ? AND user_id = ? AND type = ?",
        (post_id, auth_user.id, PostInteractionType::Dislike),
    );
    if database.affected_rows() > 0 {
        database.execute(
            "UPDATE posts SET dislikes = dislikes - 1 WHERE id = ?",
            post_id,
        );
    }
}

// MARK: Posts index
pub fn posts_index(req: &Request, ctx: &Context, _: &Path) -> Response {
    // Authorization
    // -

    // Parse request query
    let query = match req.url.query.as_ref() {
        Some(query) => match serde_urlencoded::from_str::<IndexQuery>(query) {
            Ok(query) => query,
            Err(_) => return Response::with_status(Status::BadRequest),
        },
        None => IndexQuery::default(),
    };
    if let Err(report) = query.validate() {
        return Response::with_status(Status::BadRequest).json(report);
    }

    // Get posts
    let limit = query.limit.unwrap_or(LIMIT_DEFAULT);
    let posts = ctx
        .database
        .query::<Post>(
            format!(
                "SELECT {} FROM posts WHERE text LIKE ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
                Post::columns()
            ),
            (
                format!("%{}%", query.query.unwrap_or_default().replace("%", "\\%")),
                limit,
                limit * (query.page.unwrap_or(1) - 1),
            ),
        )
        .map(|mut post| {
            post.fetch_relationships_and_update_views(ctx);
            post
        })
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

    // Create new post
    let mut post = Post {
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

    // Return new post
    post.fetch_relationships_and_update_views(ctx);
    Response::new().json(post)
}

// MARK: Posts show
pub fn posts_show(req: &Request, ctx: &Context, path: &Path) -> Response {
    let mut post = match find_post(ctx, path) {
        Some(post) => post,
        None => return not_found(req, ctx, path),
    };

    // Authorization
    // -

    // Fetch post replies
    let replies = ctx
        .database
        .query::<Post>(
            format!(
                "SELECT {} FROM posts WHERE parent_post_id = ? AND type = ? ORDER BY created_at DESC",
                Post::columns()
            ),
            (
                post.id,
                PostType::Reply,
            )
        )
        .map(|mut reply| {
            reply.fetch_relationships_and_update_views(ctx);
            reply
        })
        .collect::<Vec<_>>();
    post.replies = Some(replies);

    post.fetch_relationships_and_update_views(ctx);
    Response::new().json(post)
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

    // Update parent post counters
    if post.r#type == PostType::Reply {
        ctx.database.execute(
            "UPDATE posts SET replies = replies - 1 WHERE id = ?",
            post.parent_post_id,
        );
    }
    if post.r#type == PostType::Repost {
        ctx.database.execute(
            "UPDATE posts SET reposts = reposts - 1 WHERE id = ?",
            post.parent_post_id,
        );
    }

    // Delete post
    ctx.database
        .execute("DELETE FROM posts WHERE id = ?", post.id);

    Response::new()
}

// MARK: Posts replies
pub fn posts_replies(req: &Request, ctx: &Context, path: &Path) -> Response {
    let post = match find_post(ctx, path) {
        Some(post) => post,
        None => return not_found(req, ctx, path),
    };

    // Authorization
    // -

    // Parse request query
    let query = match req.url.query.as_ref() {
        Some(query) => match serde_urlencoded::from_str::<IndexQuery>(query) {
            Ok(query) => query,
            Err(_) => return Response::with_status(Status::BadRequest),
        },
        None => IndexQuery::default(),
    };
    if let Err(report) = query.validate() {
        return Response::with_status(Status::BadRequest).json(report);
    }

    // Get post replies
    let limit = query.limit.unwrap_or(LIMIT_DEFAULT);
    let posts = ctx
        .database
        .query::<Post>(
            format!(
                "SELECT {} FROM posts WHERE parent_post_id = ? AND text LIKE ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
                Post::columns()
            ),
            (
                post.id,
                format!("%{}%", query.query.unwrap_or_default().replace("%", "\\%")),
                limit,
                limit * (query.page.unwrap_or(1) - 1),
            ),
        )
        .map(|mut post| {
            post.fetch_relationships_and_update_views(ctx);
            post
        })
        .collect::<Vec<_>>();

    Response::new().json(posts)
}

// MARK: Post create reply
pub fn posts_create_reply(req: &Request, ctx: &Context, path: &Path) -> Response {
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

    // Create new reply post
    let now = Utc::now();
    let mut reply = Post {
        id: Uuid::now_v7(),
        r#type: PostType::Reply,
        parent_post_id: Some(post.id),
        user_id: auth_user.id,
        text: body.text,
        user: Some(auth_user.clone()),
        created_at: now,
        updated_at: now,
        ..Default::default()
    };
    ctx.database.execute(
        format!(
            "INSERT INTO posts ({}) VALUES ({})",
            Post::columns(),
            Post::values()
        ),
        reply.clone(),
    );

    // Update parent post replies counter
    ctx.database.execute(
        "UPDATE posts SET replies = replies + 1 WHERE id = ?",
        post.id,
    );

    // Return new reply
    reply.fetch_relationships_and_update_views(ctx);
    Response::new().json(reply)
}

// MARK: Posts repost
pub fn posts_repost(req: &Request, ctx: &Context, path: &Path) -> Response {
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

    // Create new repost
    let now = Utc::now();
    let mut repost = Post {
        id: Uuid::now_v7(),
        r#type: PostType::Repost,
        parent_post_id: Some(post.content_post_id()),
        user_id: auth_user.id,
        user: Some(auth_user.clone()),
        created_at: now,
        updated_at: now,
        ..Default::default()
    };
    ctx.database.execute(
        format!(
            "INSERT INTO posts ({}) VALUES ({})",
            Post::columns(),
            Post::values()
        ),
        repost.clone(),
    );

    // Update content post reposts counter
    ctx.database.execute(
        "UPDATE posts SET reposts = reposts + 1 WHERE id = ?",
        post.content_post_id(),
    );

    // Return new repost
    repost.fetch_relationships_and_update_views(ctx);
    Response::new().json(repost)
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
    remove_post_like(&ctx.database, post.content_post_id(), auth_user);
    remove_post_dislike(&ctx.database, post.content_post_id(), auth_user);

    // Create new post like interaction
    let now = Utc::now();
    let post_interaction = PostInteraction {
        id: Uuid::now_v7(),
        post_id: post.content_post_id(),
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
        "UPDATE posts SET likes = likes + 1 WHERE id = ?",
        post.content_post_id(),
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
    remove_post_like(&ctx.database, post.content_post_id(), auth_user);
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
    remove_post_like(&ctx.database, post.content_post_id(), auth_user);
    remove_post_dislike(&ctx.database, post.content_post_id(), auth_user);

    // Create new post dislike interaction
    let now = Utc::now();
    let post_interaction = PostInteraction {
        id: Uuid::now_v7(),
        post_id: post.content_post_id(),
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
        "UPDATE posts SET dislikes = dislikes + 1 WHERE id = ?",
        post.content_post_id(),
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
    remove_post_dislike(&ctx.database, post.content_post_id(), auth_user);
    Response::new()
}
