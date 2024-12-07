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
use crate::models::user::{
    is_current_password, is_unique_email, is_unique_email_or_auth_user_email, is_unique_username,
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

    ctx.database
        .query::<User>(
            format!("SELECT {} FROM users WHERE id = ? LIMIT 1", User::columns()),
            user_id,
        )
        .next()
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
        .query::<User>(format!("SELECT {} FROM users", User::columns()), ())
        .collect::<Vec<_>>();
    Ok(Response::new().json(users))
}

// MARK: Users create
pub fn users_create(req: &Request, ctx: &Context, _: &Path) -> Result<Response> {
    // Authorization
    // -

    // Parse and validate body
    #[derive(Deserialize, Validate)]
    #[validate(context(Context))]
    struct Body {
        #[validate(ascii, length(min = 1), custom(is_unique_username))]
        username: String,
        #[validate(email, custom(is_unique_email))]
        email: String,
        #[validate(ascii, length(min = 6))]
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
    if let Err(errors) = body.validate_with(ctx) {
        return Ok(Response::new().status(Status::BadRequest).json(errors));
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
    ctx.database.execute(
        format!(
            "INSERT INTO users ({}) VALUES ({})",
            User::columns(),
            User::values()
        ),
        user.clone(),
    );

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
        #[validate(context(Context))]
        struct Body {
            #[validate(
                ascii,
                length(min = 1),
                custom(is_unique_username_or_auth_user_username)
            )]
            username: String,
            #[validate(email, custom(is_unique_email_or_auth_user_email))]
            email: String,
        }
        let body = match serde_urlencoded::from_str::<Body>(&req.body) {
            Ok(body) => body,
            Err(_) => {
                return Ok(Response::new()
                    .status(Status::BadRequest)
                    .body("400 Bad Request"));
            }
        };
        if let Err(errors) = body.validate_with(ctx) {
            return Ok(Response::new().status(Status::BadRequest).json(errors));
        }

        // Update user
        user.username = body.username;
        user.email = body.email;
        user.updated_at = Utc::now();
        ctx.database.execute(
            "UPDATE users SET username = ?, email = ? WHERE id = ?",
            (user.username.clone(), user.email.clone(), user.id),
        );

        Ok(Response::new().json(user))
    } else {
        not_found(req, ctx, path)
    }
}

// MARK: Users change password
pub fn users_change_password(req: &Request, ctx: &Context, path: &Path) -> Result<Response> {
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
        #[validate(context(Context))]
        struct Body {
            #[validate(ascii, custom(is_current_password))]
            current_password: String,
            #[validate(ascii, length(min = 6))]
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
        if let Err(errors) = body.validate_with(ctx) {
            return Ok(Response::new().status(Status::BadRequest).json(errors));
        }

        // Update user
        user.password = bcrypt::hash(body.password, bcrypt::DEFAULT_COST)?;
        user.updated_at = Utc::now();
        ctx.database.execute(
            "UPDATE users SET password = ? WHERE id = ?",
            (user.password.clone(), user.id),
        );

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
            )
            .collect::<Vec<_>>();
        Ok(Response::new().json(user_sessions))
    } else {
        not_found(req, ctx, path)
    }
}
