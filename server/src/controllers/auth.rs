/*
 * Copyright (c) 2024-2025 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use base64::engine::general_purpose::STANDARD_NO_PAD as BASE64_NO_PAD;
use base64::Engine as _;
use chrono::Utc;
use const_format::formatcp;
use pbkdf2::password_verify;
use serde::Deserialize;
use small_http::{Request, Response, Status};
use validate::Report;

use crate::database::Extension;
use crate::models::{Session, User};
use crate::{api, Context, USER_AGENT_PARSER};

// MARK: Auth login
pub fn auth_login(req: &Request, ctx: &Context) -> Response {
    // Parse body
    let body = match serde_urlencoded::from_bytes::<api::AuthLoginBody>(
        req.body.as_deref().unwrap_or(&[]),
    ) {
        Ok(body) => body,
        Err(_) => {
            return Response::new()
                .status(Status::BadRequest)
                .body("400 Bad Request");
        }
    };

    // Find user by username or email
    let user = ctx
        .database
        .query::<User>(
            formatcp!(
                "SELECT {} FROM users WHERE username = ? OR email = ? LIMIT 1",
                User::columns()
            ),
            (body.logon.clone(), body.logon),
        )
        .next();
    let user = match user {
        Some(user) => user,
        None => {
            let mut report = Report::new();
            report.insert_error("logon", "Wrong username, email address or password");
            return Response::new().status(Status::Unauthorized).json(report);
        }
    };

    // Check password
    if !password_verify(&body.password, &user.password).expect("Can't verify password") {
        let mut report = Report::new();
        report.insert_error("logon", "Wrong username, email address or password");
        return Response::new().status(Status::Unauthorized).json(report);
    }

    // Get IP information from ipinfo.io
    #[derive(Deserialize)]
    struct IpInfo {
        city: String,
        country: String,
        loc: String,
    }
    let ip_address = req.client_addr.ip();
    let ip_info = match Request::with_url(format!("http://ipinfo.io/{}/json", ip_address)).fetch() {
        Ok(res) => serde_json::from_slice::<IpInfo>(&res.body).ok(),
        Err(_) => None,
    };

    // Parse user agent info
    let user_agent = req
        .headers
        .get("User-Agent")
        .map(|user_agent| USER_AGENT_PARSER.parse(user_agent));

    // Generate token
    let token = generate_random_token();

    // Create new session
    let session = Session {
        user_id: user.id,
        token: token.clone(),
        ip_address: req.client_addr.ip().to_string(),
        ip_latitude: ip_info.as_ref().and_then(|info| {
            info.loc
                .split(',')
                .next()
                .expect("Should exists")
                .parse()
                .ok()
        }),
        ip_longitude: ip_info.as_ref().and_then(|info| {
            info.loc
                .split(',')
                .nth(1)
                .expect("Should exists")
                .parse()
                .ok()
        }),
        ip_country: ip_info.as_ref().map(|info| info.country.clone()),
        ip_city: ip_info.as_ref().map(|info| info.city.clone()),
        client_name: user_agent.as_ref().map(|ua| ua.client.family.to_string()),
        client_version: user_agent.as_ref().and_then(|ua| ua.client.version.clone()),
        client_os: user_agent.as_ref().map(|ua| ua.os.family.to_string()),
        ..Default::default()
    };
    ctx.database.insert_session(session.clone());

    // Return session
    Response::new().json(api::AuthLoginResponse {
        token,
        session: session.into(),
        user: user.into(),
    })
}

pub fn generate_random_token() -> String {
    let mut token_bytes = [0u8; 256];
    getrandom::fill(&mut token_bytes).expect("Can't get random bytes");
    BASE64_NO_PAD.encode(token_bytes)
}

// MARK: Auth validate
pub fn auth_validate(_: &Request, ctx: &Context) -> Response {
    Response::new().json(api::AuthValidateResponse {
        session: ctx.auth_session.clone().expect("Should be authed").into(),
        user: ctx.auth_user.clone().expect("Should be authed").into(),
    })
}

// MARK: Auth logout
pub fn auth_logout(_: &Request, ctx: &Context) -> Response {
    // Expire session
    ctx.database.execute(
        "UPDATE sessions SET expires_at = ? WHERE token = ?",
        (
            Utc::now(),
            ctx.auth_session.as_ref().expect("Not authed").token.clone(),
        ),
    );
    Response::new().status(Status::Ok)
}

#[cfg(test)]
mod test {
    use pbkdf2::password_hash;
    use small_http::Method;

    use super::*;
    use crate::database::Extension;
    use crate::models::UserRole;
    use crate::router;
    use crate::test_utils::create_user_session;

    // MARK: Test Auth login
    #[test]
    fn test_auth_login() {
        let ctx = Context::with_test_database();
        let router = router(ctx.clone());

        ctx.database.insert_user(User {
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password: password_hash("password"),
            ..Default::default()
        });

        // Login with username
        let req = Request::with_url("http://localhost/auth/login")
            .method(Method::Post)
            .body("logon=test&password=password");
        let res = router.handle(&req);
        assert_eq!(res.status, Status::Ok);
        let json = serde_json::from_slice::<api::AuthLoginResponse>(&res.body).unwrap();
        assert!(!json.token.is_empty());
        assert_eq!(json.user.username, "test");

        // Login with email
        let req = Request::with_url("http://localhost/auth/login")
            .method(Method::Post)
            .body("logon=test@example.com&password=password");
        let res = router.handle(&req);
        assert_eq!(res.status, Status::Ok);
        let json = serde_json::from_slice::<api::AuthLoginResponse>(&res.body).unwrap();
        assert!(!json.token.is_empty());
        assert_eq!(json.user.email, "test@example.com");

        // Login with wrong username
        let req = Request::with_url("http://localhost/auth/login")
            .method(Method::Post)
            .body("logon=wrongtest&password=password");
        let res = router.handle(&req);
        assert_eq!(res.status, Status::Unauthorized);
        let report = serde_json::from_slice::<Report>(&res.body).unwrap();
        assert_eq!(
            report.get_errors("logon").unwrap().as_slice(),
            &["Wrong username, email address or password".to_string()]
        );

        // Login with wrong password
        let req = Request::with_url("http://localhost/auth/login")
            .method(Method::Post)
            .body("logon=test&password=wrongpassword");
        let res = router.handle(&req);
        assert_eq!(res.status, Status::Unauthorized);
        let report = serde_json::from_slice::<Report>(&res.body).unwrap();
        assert_eq!(
            report.get_errors("logon").unwrap().as_slice(),
            &["Wrong username, email address or password".to_string()]
        );
    }

    // MARK: Test Auth validate
    #[test]
    fn test_auth_validate() {
        let ctx = Context::with_test_database();
        let router = router(ctx.clone());
        let (user, session) = create_user_session(&ctx, UserRole::Normal);

        // Validate session
        let req = Request::with_url("http://localhost/auth/validate")
            .header("Authorization", format!("Bearer {}", session.token));
        let res = router.handle(&req);
        assert_eq!(res.status, Status::Ok);
        let json = serde_json::from_slice::<api::AuthValidateResponse>(&res.body).unwrap();
        assert_eq!(json.user.username, user.username);

        // Validate session with wrong token
        let req = Request::with_url("http://localhost/auth/validate")
            .header("Authorization", "Bearer wrong_token");
        let res = router.handle(&req);
        assert_eq!(res.status, Status::Unauthorized);
    }

    // MARK: Test Auth logout
    #[test]
    fn test_auth_logout() {
        let ctx = Context::with_test_database();
        let router = router(ctx.clone());
        let (_, session) = create_user_session(&ctx, UserRole::Normal);

        // Logout session
        let req = Request::with_url("http://localhost/auth/logout")
            .method(Method::Put)
            .header("Authorization", format!("Bearer {}", session.token));
        let res = router.handle(&req);
        assert_eq!(res.status, Status::Ok);

        // Validate session should be expired
        let req = Request::with_url("http://localhost/auth/validate")
            .header("Authorization", format!("Bearer {}", session.token));
        let res = router.handle(&req);
        assert_eq!(res.status, Status::Unauthorized);
    }
}
