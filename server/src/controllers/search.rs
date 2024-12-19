/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use http::{Request, Response, Status};
use router::Path;
use serde_json::json;
use validate::Validate;

use crate::consts::LIMIT_DEFAULT;
use crate::models::{IndexQuery, Post, User};
use crate::Context;

pub fn search(req: &Request, ctx: &Context, _: &Path) -> Response {
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
    let limit = query.limit.unwrap_or(LIMIT_DEFAULT);

    // Search users
    let users = ctx
        .database
        .query::<User>(
            format!(
                "SELECT {} FROM users WHERE username LIKE ? LIMIT ? OFFSET ?",
                User::columns()
            ),
            (
                format!(
                    "%{}%",
                    query.query.clone().unwrap_or_default().replace("%", "\\%")
                ),
                limit,
                limit * (query.page.unwrap_or(1) - 1),
            ),
        )
        .collect::<Vec<_>>();

    // Search posts
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
            post.fetch_relationships(ctx);
            post
        })
        .collect::<Vec<_>>();

    // Return response
    Response::new().json(json!({
        "users": users,
        "posts": posts,
    }))
}
