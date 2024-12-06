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
use crate::models::user::{
    is_unique_email, is_unique_email_or_auth_user_email, is_unique_username,
    is_unique_username_or_auth_user_username,
};
use crate::models::{Session, User, UserRole};
use crate::Context;

// MARK: Helpers
fn get_user(ctx: &Context, path: &Path) -> Option<User> {
    let user_id = match path.get("user_id").unwrap().parse::<Uuid>() {
        Ok(id) => id,
        Err(_) => return None,
    };

    match ctx
        .database
        .query::<User>(
            format!("SELECT {} FROM users WHERE id = ? LIMIT 1", User::columns()),
            user_id,
        )
        .unwrap()
        .next()
    {
        Some(Ok(user)) => Some(user),
        _ => None,
    }
}

// MARK: Users revoke
pub fn users_index(_: &Request, ctx: &Context, _: &Path) -> Result<Response> {
    // Authorization
    let auth_user = ctx.auth_user.as_ref().unwrap();
    if !(auth_user.role == UserRole::Admin) {
        return Ok(Response::new()
            .status(Status::Unauthorized)
            .body("401 Unauthorized"));
    }

    let users = ctx
        .database
        .query::<User>(format!("SELECT {} FROM users", User::columns()), ())?
        .collect::<Result<Vec<_>, sqlite::Error>>()?;
    Ok(Response::new().json(users))
}

// MARK: Users create
pub fn users_create(req: &Request, ctx: &Context, _: &Path) -> Result<Response> {
    // Authorization
    // -

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
        role: UserRole::Normal,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    ctx.database
        .query::<()>(
            format!(
                "INSERT INTO users ({}) VALUES ({})",
                User::columns(),
                User::values()
            ),
            user.clone(),
        )?
        .next();

    Ok(Response::new().json(user))
}

// MARK: Users show
pub fn users_show(req: &Request, ctx: &Context, path: &Path) -> Result<Response> {
    let user = get_user(ctx, path);
    if let Some(user) = user {
        // Authorization
        let auth_user = ctx.auth_user.as_ref().unwrap();
        if !(user.id == auth_user.id || auth_user.role == UserRole::Admin) {
            return Ok(Response::new()
                .status(Status::Unauthorized)
                .body("401 Unauthorized"));
        }

        Ok(Response::new().json(user))
    } else {
        not_found(req, ctx, path)
    }
}

// MARK: Users update
pub fn users_update(req: &Request, ctx: &Context, path: &Path) -> Result<Response> {
    let user = get_user(ctx, path);
    if let Some(mut user) = user {
        // Authorization
        let auth_user = ctx.auth_user.as_ref().unwrap();
        if !(user.id == auth_user.id || auth_user.role == UserRole::Admin) {
            return Ok(Response::new()
                .status(Status::Unauthorized)
                .body("401 Unauthorized"));
        }

        // Parse and validate body
        #[derive(Deserialize, Validate)]
        #[garde(context(Context))]
        struct Body {
            #[garde(
                ascii,
                length(min = 1),
                custom(is_unique_username_or_auth_user_username)
            )]
            username: String,
            #[garde(email, custom(is_unique_email_or_auth_user_email))]
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

        // Update user
        user.username = body.username;
        user.email = body.email;
        user.password = bcrypt::hash(body.password, bcrypt::DEFAULT_COST)?;
        user.updated_at = Utc::now();
        ctx.database
            .query::<()>(
                format!("UPDATE users SET {} WHERE id = ?", User::sets()),
                user.clone(),
            )?
            .next();

        Ok(Response::new().json(user))
    } else {
        not_found(req, ctx, path)
    }
}

// MARK: Users sessions
pub fn users_sessions(req: &Request, ctx: &Context, path: &Path) -> Result<Response> {
    let user = get_user(ctx, path);
    if let Some(user) = user {
        // Authorization
        let auth_user = ctx.auth_user.as_ref().unwrap();
        if !(user.id == auth_user.id || auth_user.role == UserRole::Admin) {
            return Ok(Response::new()
                .status(Status::Unauthorized)
                .body("401 Unauthorized"));
        }

        let user_sessions = ctx
            .database
            .query::<Session>(
                format!(
                    "SELECT {} FROM sessions WHERE user_id = ?",
                    Session::columns()
                ),
                user.id,
            )?
            .collect::<Result<Vec<_>, sqlite::Error>>()?;
        Ok(Response::new().json(user_sessions))
    } else {
        not_found(req, ctx, path)
    }
}
