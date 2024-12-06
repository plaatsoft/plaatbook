/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use anyhow::Result;
use chrono::Utc;
use http::{Request, Response, Status};
use router::Path;
use uuid::Uuid;

use crate::controllers::not_found;
use crate::models::User;
use crate::Context;

pub fn users_index(_: &Request, ctx: &Context, _: &Path) -> Result<Response> {
    // Get users
    let users = ctx
        .database
        .query::<User>(format!("SELECT {} FROM users", User::columns()), ())?
        .collect::<Result<Vec<_>, sqlite::Error>>()?;
    Ok(Response::new().json(users))
}

pub fn users_store(_: &Request, ctx: &Context, _: &Path) -> Result<Response> {
    // Create a new user
    let user = User {
        id: Uuid::now_v7(),
        username: "plaatsoft".to_string(),
        email: "info@plaatsoft.nl".to_string(),
        password: "password".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    ctx.database
        .query::<()>(
            format!(
                "INSERT INTO users ({}) VALUES ({})",
                User::columns(),
                User::params()
            ),
            user.clone(),
        )?
        .next();

    Ok(Response::new().json(user))
}

pub fn users_show(req: &Request, ctx: &Context, path: &Path) -> Result<Response> {
    // Parse user id from url
    let user_id = match path.get("user_id").expect("Should be some").parse::<Uuid>() {
        Ok(id) => id,
        Err(_) => {
            return Ok(Response::new()
                .status(Status::BadRequest)
                .body("400 Bad Request"));
        }
    };

    // Get user
    let user = ctx
        .database
        .query::<User>(
            format!("SELECT {} FROM users WHERE id = ? LIMIT 1", User::columns()),
            user_id,
        )?
        .next();

    if let Some(Ok(user)) = user {
        Ok(Response::new().json(user))
    } else {
        not_found(req, ctx, path)
    }
}
