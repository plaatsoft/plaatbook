/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use http::{Request, Response};

use crate::models::{self, Session};
use crate::Context;

pub fn auth_layer(req: &Request, ctx: &mut Context) -> Option<Response> {
    // Get token from Authorization header
    let authorization = match req
        .headers
        .get("Authorization")
        .or(req.headers.get("authorization"))
    {
        Some(authorization) => authorization,
        None => {
            return Some(
                Response::new()
                    .status(http::Status::Unauthorized)
                    .body("401 Unauthorized"),
            );
        }
    };
    let token = authorization[7..].trim().to_string();

    // Get active session by token
    let session = ctx
        .database
        .query::<models::Session>(
            format!(
                "SELECT {} FROM sessions WHERE token = ? AND expires_at > ? LIMIT 1",
                Session::columns()
            ),
            (token, chrono::Utc::now()),
        )
        .next();
    let session = match session {
        Some(session) => session,
        None => {
            return Some(
                Response::new()
                    .status(http::Status::Unauthorized)
                    .body("401 Unauthorized"),
            );
        }
    };

    // Get user by session user_id
    ctx.auth_user = ctx
        .database
        .query::<models::User>(
            format!(
                "SELECT {} FROM users WHERE id = ? LIMIT 1",
                models::User::columns()
            ),
            session.user_id,
        )
        .next();
    ctx.auth_session = Some(session);

    None
}
