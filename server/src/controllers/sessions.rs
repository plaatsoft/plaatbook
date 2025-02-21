/*
 * Copyright (c) 2024-2025 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use chrono::Utc;
use const_format::formatcp;
use small_http::{Request, Response, Status};
use uuid::Uuid;
use validate::Validate;

use crate::controllers::not_found;
use crate::models::{IndexQuery, Session, User, UserRole};
use crate::{api, Context};

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
            formatcp!(
                "SELECT {} FROM sessions WHERE id = ? LIMIT 1",
                Session::columns()
            ),
            session_id,
        )
        .next()
}

// MARK: Sessions index
pub fn sessions_index(req: &Request, ctx: &Context) -> Response {
    // Authorization
    let auth_user = ctx.auth_user.as_ref().expect("Not authed");
    if !(auth_user.role == UserRole::Admin) {
        return Response::new()
            .status(Status::Unauthorized)
            .body("401 Unauthorized");
    }

    // Parse index query
    let query = match req.url.query() {
        Some(query) => match serde_urlencoded::from_str::<IndexQuery>(query) {
            Ok(query) => query,
            Err(_) => return Response::with_status(Status::BadRequest),
        },
        None => IndexQuery::default(),
    };
    if let Err(report) = query.validate() {
        return Response::with_status(Status::BadRequest).json(report);
    }

    let total = ctx
        .database
        .query::<i64>("SELECT COUNT(id) FROM sessions", ())
        .next()
        .expect("Can't count sessions");
    let sessions = ctx
        .database
        .query::<Session>(
            formatcp!(
                "SELECT {} FROM sessions ORDER BY expires_at DESC LIMIT ? OFFSET ?",
                Session::columns()
            ),
            (query.limit, (query.page - 1) * query.limit),
        )
        .map(|mut session| {
            session.user = ctx
                .database
                .query::<User>(
                    formatcp!("SELECT {} FROM users WHERE id = ? LIMIT 1", User::columns()),
                    session.user_id,
                )
                .next();
            session
        })
        .map(Into::<api::Session>::into)
        .collect::<Vec<_>>();
    Response::new().json(api::SessionIndexResponse {
        pagination: api::Pagination {
            total,
            page: query.page,
            limit: query.limit,
        },
        data: sessions,
    })
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
    Response::new().json(Into::<api::Session>::into(session))
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

#[cfg(test)]
mod test {
    use small_http::Method;

    use super::*;
    use crate::controllers::auth::generate_random_token;
    use crate::database::Extension;
    use crate::router;
    use crate::test_utils::create_user_session;

    // MARK: Test Sessions index
    #[test]
    fn test_sessions_index() {
        let ctx = Context::with_test_database();
        let router = router(ctx.clone());
        let (user, session) = create_user_session(&ctx, UserRole::Admin);

        for _ in 0..10 {
            ctx.database.insert_session(Session {
                user_id: user.id,
                token: generate_random_token(),
                ..Default::default()
            });
        }

        let req = Request::with_url("http://localhost/sessions")
            .header("Authorization", format!("Bearer {}", session.token));
        let res = router.handle(&req);
        assert_eq!(res.status, Status::Ok);
        let res = serde_json::from_slice::<api::SessionIndexResponse>(&res.body).unwrap();
        assert_eq!(res.pagination.total, 10 + 1);
    }

    // MARK: Test Sessions show
    #[test]
    fn test_sessions_show() {
        let ctx = Context::with_test_database();
        let router = router(ctx.clone());

        // Show your own session
        let (_, user_session) = create_user_session(&ctx, UserRole::Normal);
        let req = Request::with_url(format!("http://localhost/sessions/{}", user_session.id))
            .header("Authorization", format!("Bearer {}", user_session.token));
        let res = router.handle(&req);
        assert_eq!(res.status, Status::Ok);
        let res = serde_json::from_slice::<api::Session>(&res.body).unwrap();
        assert_eq!(res.id, user_session.id);

        // Admin show user session
        let (_, admin_session) = create_user_session(&ctx, UserRole::Admin);
        let req = Request::with_url(format!("http://localhost/sessions/{}", user_session.id))
            .header("Authorization", format!("Bearer {}", admin_session.token));
        let res = router.handle(&req);
        assert_eq!(res.status, Status::Ok);
        let res = serde_json::from_slice::<api::Session>(&res.body).unwrap();
        assert_eq!(res.id, user_session.id);

        // Unauthorized show user session
        let (_, other_user_session) = create_user_session(&ctx, UserRole::Normal);
        let req = Request::with_url(format!("http://localhost/sessions/{}", user_session.id))
            .header(
                "Authorization",
                format!("Bearer {}", other_user_session.token),
            );
        let res = router.handle(&req);
        assert_eq!(res.status, Status::Unauthorized);
    }

    // MARK: Test Sessions revoke
    #[test]
    fn test_sessions_revoke() {
        let ctx = Context::with_test_database();
        let router = router(ctx.clone());

        // Revoke your own session
        let (_, user_session) = create_user_session(&ctx, UserRole::Normal);
        let req = Request::with_url(format!("http://localhost/sessions/{}", user_session.id))
            .method(Method::Delete)
            .header("Authorization", format!("Bearer {}", user_session.token));
        let res = router.handle(&req);
        assert_eq!(res.status, Status::Ok);

        // Check if session is revoked
        let req = Request::with_url(format!("http://localhost/sessions/{}", user_session.id))
            .header("Authorization", format!("Bearer {}", user_session.token));
        let res = router.handle(&req);
        assert_eq!(res.status, Status::Unauthorized);

        // Admin revoke user session
        let (_, admin_session) = create_user_session(&ctx, UserRole::Admin);
        let (_, user_session) = create_user_session(&ctx, UserRole::Normal);
        let req = Request::with_url(format!("http://localhost/sessions/{}", user_session.id))
            .method(Method::Delete)
            .header("Authorization", format!("Bearer {}", admin_session.token));
        let res = router.handle(&req);
        assert_eq!(res.status, Status::Ok);

        // Check if session is revoked
        let req = Request::with_url(format!("http://localhost/sessions/{}", user_session.id))
            .header("Authorization", format!("Bearer {}", user_session.token));
        let res = router.handle(&req);
        assert_eq!(res.status, Status::Unauthorized);
    }
}
