/*
 * Copyright (c) 2024-2025 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use http::{Request, Response};
use time::DateTime;

use crate::models::{self, Session};
use crate::Context;

// MARK: Auth optional
pub fn auth_optional_pre_layer(req: &Request, ctx: &mut Context) -> Option<Response> {
    // Get token from Authorization header
    let authorization = match req
        .headers
        .get("Authorization")
        .or(req.headers.get("authorization"))
    {
        Some(authorization) => authorization,
        None => return None,
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
            (token, DateTime::now()),
        )
        .next();
    let session = match session {
        Some(session) => session,
        None => return None,
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

// MARK: Auth required
pub fn auth_required_pre_layer(req: &Request, ctx: &mut Context) -> Option<Response> {
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
            (token, DateTime::now()),
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

// MARK: Tests
#[cfg(test)]
mod test {
    use std::time::Duration;

    use models::User;
    use time::DateTime;

    use super::*;
    use crate::router;

    #[test]
    fn test_unauthed() {
        let ctx = Context::with_test_database();
        let router = router(ctx.clone());

        // Create a test user
        let user = User::default();
        ctx.database.execute(
            format!(
                "INSERT INTO users ({}) VALUES ({})",
                User::columns(),
                User::values()
            ),
            user.clone(),
        );

        let res = router.handle(&Request::with_url("http://localhost/auth/validate"));
        assert_eq!(res.status, http::Status::Unauthorized);
    }

    #[test]
    fn test_authed() {
        let ctx = Context::with_test_database();
        let router = router(ctx.clone());

        // Create a test user and session
        let user = User::default();
        ctx.database.execute(
            format!(
                "INSERT INTO users ({}) VALUES ({})",
                User::columns(),
                User::values()
            ),
            user.clone(),
        );
        let session = Session {
            user_id: user.id,
            token: "test".to_string(),
            expires_at: DateTime::now() + Duration::from_secs(60 * 60),
            ..Default::default()
        };
        ctx.database.execute(
            format!(
                "INSERT INTO sessions ({}) VALUES ({})",
                Session::columns(),
                Session::values()
            ),
            session.clone(),
        );

        // Add Authorization header to request
        let req = Request::with_url("http://localhost/auth/validate")
            .header("Authorization", format!("Bearer {}", session.token));
        let res = router.handle(&req);
        assert_eq!(res.status, http::Status::Ok);
    }
}
