/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use anyhow::Result;
use chrono::Utc;
use garde::Validate;
use http::{Request, Response, Status};
use router::Path;
use serde::Deserialize;
use uuid::Uuid;

use crate::controllers::not_found;
use crate::models::user::{is_unique_email, is_unique_username};
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

pub fn users_store(req: &Request, ctx: &Context, _: &Path) -> Result<Response> {
    // Parse and validate body
    #[derive(Deserialize, Validate)]
    #[garde(context(Context))]
    struct Body {
        #[garde(ascii, length(min = 1), custom(is_unique_username))]
        username: String,
        #[garde(email, custom(is_unique_email))]
        email: String,
        #[garde(ascii, length(min = 6))]
        password: String,
    }
    let body = match serde_urlencoded::from_str::<Body>(&req.body) {
        Ok(body) => body,
        Err(_) => {
            return Ok(Response::new()
                .status(Status::BadRequest)
                .body("400 Bad Request"));
        }
    };
    if let Err(err) = body.validate_with(ctx) {
        return Ok(Response::new().status(Status::BadRequest).json(err));
    }

    // Create a new user
    let user = User {
        id: Uuid::now_v7(),
        username: body.username,
        email: body.email,
        password: bcrypt::hash(body.password, bcrypt::DEFAULT_COST)?,
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
    let user_id = match path.get("user_id").unwrap().parse::<Uuid>() {
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
