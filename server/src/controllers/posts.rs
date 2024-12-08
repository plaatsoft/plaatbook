/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use anyhow::Result;
use chrono::Utc;
use http::{Request, Response, Status};
use router::Path;
use serde::Deserialize;
use uuid::Uuid;
use validate::Validate;

use crate::controllers::not_found;
use crate::models::{Post, User, UserRole};
use crate::Context;

// MARK: Helpers
fn get_post(ctx: &Context, path: &Path) -> Option<Post> {
    let post_id = match path.get("post_id").unwrap().parse::<Uuid>() {
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

// MARK: Posts index
pub fn posts_index(_: &Request, ctx: &Context, _: &Path) -> Result<Response> {
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
        .map(|mut post| {
            post.user = ctx
                .database
                .query::<User>(
                    format!("SELECT {} FROM users WHERE id = ? LIMIT 1", User::columns()),
                    post.user_id,
                )
                .next();
            post
        })
        .collect::<Vec<_>>();

    Ok(Response::new().json(posts))
}

// MARK: Posts create
pub fn posts_create(req: &Request, ctx: &Context, _: &Path) -> Result<Response> {
    // Authorization
    if ctx.auth_user.is_none() {
        return Ok(Response::new()
            .status(Status::Unauthorized)
            .body("401 Unauthorized"));
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
            return Ok(Response::new()
                .status(Status::BadRequest)
                .body("400 Bad Request"));
        }
    };
    if let Err(errors) = body.validate() {
        return Ok(Response::new().status(Status::BadRequest).json(errors));
    }

    // Create a new post
    let post = Post {
        user_id: ctx.auth_user.as_ref().unwrap().id,
        text: body.text,
        user: ctx.auth_user.clone(),
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

    Ok(Response::new().json(post))
}

// MARK: Posts show
pub fn posts_show(req: &Request, ctx: &Context, path: &Path) -> Result<Response> {
    let post = get_post(ctx, path);
    if let Some(mut post) = post {
        // Authorization
        // -

        post.user = ctx
            .database
            .query::<User>(
                format!("SELECT {} FROM users WHERE id = ? LIMIT 1", User::columns()),
                post.user_id,
            )
            .next();

        Ok(Response::new().json(post))
    } else {
        not_found(req, ctx, path)
    }
}

// MARK: Posts update
pub fn posts_update(req: &Request, ctx: &Context, path: &Path) -> Result<Response> {
    let post = get_post(ctx, path);
    if let Some(mut post) = post {
        // Authorization
        let auth_post = ctx.auth_user.as_ref().unwrap();
        if !(post.user_id == auth_post.id || auth_post.role == UserRole::Admin) {
            return Ok(Response::new()
                .status(Status::Unauthorized)
                .body("401 Unauthorized"));
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
                return Ok(Response::new()
                    .status(Status::BadRequest)
                    .body("400 Bad Request"));
            }
        };
        if let Err(errors) = body.validate() {
            return Ok(Response::new().status(Status::BadRequest).json(errors));
        }

        // Update post
        post.text = body.text;
        post.updated_at = Utc::now();
        ctx.database.execute(
            "UPDATE posts SET text = ? WHERE id = ?",
            (post.text.clone(), post.id),
        );

        Ok(Response::new().json(post))
    } else {
        not_found(req, ctx, path)
    }
}

// MARK: Posts delete
pub fn posts_delete(req: &Request, ctx: &Context, path: &Path) -> Result<Response> {
    let post = get_post(ctx, path);
    if let Some(post) = post {
        // Authorization
        let auth_post = ctx.auth_user.as_ref().unwrap();
        if !(post.user_id == auth_post.id || auth_post.role == UserRole::Admin) {
            return Ok(Response::new()
                .status(Status::Unauthorized)
                .body("401 Unauthorized"));
        }

        // Delete post
        ctx.database
            .execute("DELETE FROM posts WHERE id = ?", post.id);

        Ok(Response::new())
    } else {
        not_found(req, ctx, path)
    }
}
