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
use crate::models::{Session, UserRole};
use crate::Context;

// MARK: Helpers
fn get_session(ctx: &Context, path: &Path) -> Option<Session> {
    let session_id = match path.get("session_id").unwrap().parse::<Uuid>() {
        Ok(id) => id,
        Err(_) => return None,
    };
    match ctx
        .database
        .query::<Session>(
            format!(
                "SELECT {} FROM sessions WHERE id = ? LIMIT 1",
                Session::columns()
            ),
            session_id,
        )
        .unwrap()
        .next()
    {
        Some(Ok(session)) => Some(session),
        _ => None,
    }
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
        .query::<Session>(format!("SELECT {} FROM sessions", Session::columns()), ())?
        .collect::<Result<Vec<_>, sqlite::Error>>()?;
    Ok(Response::new().json(sessions))
}

// MARK: Sessions show
pub fn sessions_show(req: &Request, ctx: &Context, path: &Path) -> Result<Response> {
    let session = get_session(ctx, path);
    if let Some(session) = session {
        // Authorization
        let auth_user = ctx.auth_user.as_ref().unwrap();
        if !(session.user_id == auth_user.id || auth_user.role == UserRole::Admin) {
            return Ok(Response::new()
                .status(Status::Unauthorized)
                .body("401 Unauthorized"));
        }

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

        ctx.database
            .query::<()>(
                "UPDATE sessions SET expired_at = ? WHERE id = ?",
                (Utc::now(), session.id),
            )
            .unwrap();
        Ok(Response::new().status(Status::Ok))
    } else {
        not_found(req, ctx, path)
    }
}
