/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use chrono::Utc;
use http::{Request, Response, Status};
use uuid::Uuid;

use crate::controllers::not_found;
use crate::models::{Session, User, UserRole};
use crate::Context;

// MARK: Helpers
fn find_session(req: &Request, ctx: &Context) -> Option<Session> {
    let session_id = match req
        .params
        .get("session_id")
        .expect("Should be some")
        .parse::<Uuid>()
    {
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
pub fn sessions_index(_: &Request, ctx: &Context) -> Response {
    // Authorization
    let auth_user = ctx.auth_user.as_ref().expect("Not authed");
    if !(auth_user.role == UserRole::Admin) {
        return Response::new()
            .status(Status::Unauthorized)
            .body("401 Unauthorized");
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
    Response::new().json(sessions)
}

// MARK: Sessions show
pub fn sessions_show(req: &Request, ctx: &Context) -> Response {
    let mut session = match find_session(req, ctx) {
        Some(session) => session,
        None => return not_found(req, ctx),
    };

    // Authorization
    let auth_user = ctx.auth_user.as_ref().expect("Not authed");
    if !(session.user_id == auth_user.id || auth_user.role == UserRole::Admin) {
        return Response::new()
            .status(Status::Unauthorized)
            .body("401 Unauthorized");
    }

    // Return session
    session.fetch_relationships(ctx);
    Response::new().json(session)
}

// MARK: Sessions revoke
pub fn sessions_revoke(req: &Request, ctx: &Context) -> Response {
    let session = match find_session(req, ctx) {
        Some(session) => session,
        None => return not_found(req, ctx),
    };

    // Authorization
    let auth_user = ctx.auth_user.as_ref().expect("Not authed");
    if !(session.user_id == auth_user.id || auth_user.role == UserRole::Admin) {
        return Response::new()
            .status(Status::Unauthorized)
            .body("401 Unauthorized");
    }

    ctx.database.execute(
        "UPDATE sessions SET expires_at = ? WHERE id = ?",
        (Utc::now(), session.id),
    );
    Response::new()
}
