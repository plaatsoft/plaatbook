/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use http::{Request, Response, Status};
use router::Path;
use serde::Deserialize;
use serde_json::json;
use validate::Validate;

use crate::models::{Post, User};
use crate::Context;

pub fn search(req: &Request, ctx: &Context, _: &Path) -> Response {
    // Parse query get variable
    #[derive(Deserialize, Validate)]
    struct Query {
        #[serde(rename = "q")]
        #[validate(length(min = 1, max = 255))]
        query: String,
    }
    let query = match serde_urlencoded::from_str::<Query>(match req.url.query.as_ref() {
        Some(query) => query,
        None => {
            return Response::new()
                .status(Status::BadRequest)
                .body("400 Bad Request")
        }
    }) {
        Ok(query) => query,
        Err(_) => {
            return Response::new()
                .status(Status::BadRequest)
                .body("400 Bad Request")
        }
    };

    // Search users
    let query = format!("%{}%", query.query);
    let users = ctx
        .database
        .query::<User>(
            format!(
                "SELECT {} FROM users WHERE username LIKE ?",
                User::columns()
            ),
            query.clone(),
        )
        .collect::<Vec<_>>();

    // Search posts
    let posts = ctx
        .database
        .query::<Post>(
            format!("SELECT {} FROM posts WHERE text LIKE ?", Post::columns()),
            query,
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

    Response::new().json(json!({
        "users": users,
        "posts": posts,
    }))
}
