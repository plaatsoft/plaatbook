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
use crate::models::{Session, User, UserRole};
use crate::Context;

// MARK: Helpers
fn get_session(ctx: &Context, path: &Path) -> Option<Session> {
    let session_id = match path.get("session_id").unwrap().parse::<Uuid>() {
        Ok(id) => id,
        Err(_) => return None,
    };
    ctx.database
        .query::<Session>(
            format!(
                "SELECT {} FROM sessions WHERE id = ? LIMIT 1",
                Session::columns()
            ),
            session_id,
        )
        .next()
}

// MARK: Sessions index
pub fn sessions_index(_: &Request, ctx: &Context, _: &Path) -> Result<Response> {
    // Authorization
    let auth_user = ctx.auth_user.as_ref().unwrap();
    if !(auth_user.role == UserRole::Admin) {
        return Ok(Response::new()
            .status(Status::Unauthorized)
            .body("401 Unauthorized"));
    }

    let sessions = ctx
        .database
        .query::<Session>(
            format!(
                "SELECT {} FROM sessions ORDER BY expires_at DESC",
                Session::columns()
            ),
            (),
        )
        .map(|mut session| {
            session.user = ctx
                .database
                .query::<User>(
                    format!("SELECT {} FROM users WHERE id = ? LIMIT 1", User::columns()),
                    session.user_id,
                )
                .next();
            session
        })
        .collect::<Vec<_>>();
    Ok(Response::new().json(sessions))
}

// MARK: Sessions show
pub fn sessions_show(req: &Request, ctx: &Context, path: &Path) -> Result<Response> {
    let session = get_session(ctx, path);
    if let Some(mut session) = session {
        // Authorization
        let auth_user = ctx.auth_user.as_ref().unwrap();
        if !(session.user_id == auth_user.id || auth_user.role == UserRole::Admin) {
            return Ok(Response::new()
                .status(Status::Unauthorized)
                .body("401 Unauthorized"));
        }

        session.user = ctx
            .database
            .query::<User>(
                format!("SELECT {} FROM users WHERE id = ? LIMIT 1", User::columns()),
                session.user_id,
            )
            .next();

        Ok(Response::new().json(session))
    } else {
        not_found(req, ctx, path)
    }
}

// MARK: Sessions revoke
pub fn sessions_revoke(req: &Request, ctx: &Context, path: &Path) -> Result<Response> {
    let session = get_session(ctx, path);
    if let Some(session) = session {
        // Authorization
        let auth_user = ctx.auth_user.as_ref().unwrap();
        if !(session.user_id == auth_user.id || auth_user.role == UserRole::Admin) {
            return Ok(Response::new()
                .status(Status::Unauthorized)
                .body("401 Unauthorized"));
        }

        ctx.database.execute(
            "UPDATE sessions SET expires_at = ? WHERE id = ?",
            (Utc::now(), session.id),
        );
        Ok(Response::new())
    } else {
        not_found(req, ctx, path)
    }
}
