/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use chrono::Utc;
use http::{Request, Response, Status};
use pbkdf2::password_verify;
use router::Path;
use serde::Deserialize;
use useragent::UserAgentParser;

use crate::models::{Session, User};
use crate::{api, Context};

lazy_static::lazy_static! {
    static ref USER_AGENT_PARSER: UserAgentParser = UserAgentParser::new();
}

// MARK: Auth login
pub fn auth_login(req: &Request, ctx: &Context, _: &Path) -> Response {
    // Parse body
    #[derive(Deserialize)]
    struct Body {
        logon: String,
        password: String,
    }
    let body = match serde_urlencoded::from_str::<Body>(&req.body) {
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
            format!(
                "SELECT {} FROM users WHERE username = ? OR email = ? LIMIT 1",
                User::columns()
            ),
            (body.logon.clone(), body.logon),
        )
        .next();
    let user = match user {
        Some(user) => user,
        None => {
            return Response::new()
                .status(Status::Unauthorized)
                .body("Wrong username, email address or password");
        }
    };

    // Check password
    if !password_verify(&body.password, &user.password).expect("Can't verify password") {
        return Response::new()
            .status(Status::Unauthorized)
            .body("Wrong username, email address or password");
    }

    // Get IP information from ipinfo.io
    #[derive(Deserialize)]
    struct IpInfo {
        city: String,
        country: String,
        loc: String,
    }
    let ip_info = match http::fetch(Request::with_url("http://ipinfo.io/json")) {
        Ok(res) => serde_json::from_str::<IpInfo>(&res.body).ok(),
        Err(_) => None,
    };

    // Parse user agent info
    let user_agent = req
        .headers
        .get("User-Agent")
        .map(|user_agent| USER_AGENT_PARSER.parse(user_agent));

    // Generate token
    let mut token_bytes = [0u8; 256];
    getrandom::getrandom(&mut token_bytes).expect("Can't get random bytes");
    let token = base64::encode(&token_bytes, true);

    // Create new session
    let session = Session {
        user_id: user.id,
        token: token.clone(),
        ip_address: req
            .client_addr
            .expect("Should have client address")
            .ip()
            .to_string(),
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
        client_version: user_agent.as_ref().map(|ua| {
            format!(
                "{}.{}",
                ua.client.major.as_ref().unwrap_or(&"0".to_string()),
                ua.client.minor.as_ref().unwrap_or(&"0".to_string())
            )
        }),
        client_os: user_agent.as_ref().map(|ua| ua.os.family.to_string()),
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

    // Return session
    Response::new().json(api::AuthLoginResponse {
        token,
        session: session.into(),
        user: user.into(),
    })
}

// MARK: Auth validate
pub fn auth_validate(_: &Request, ctx: &Context, _: &Path) -> Response {
    Response::new().json(api::AuthValidateResponse {
        session: ctx.auth_session.clone().expect("Should be authed").into(),
        user: ctx.auth_user.clone().expect("Should be authed").into(),
    })
}

// MARK: Auth logout
pub fn auth_logout(_: &Request, ctx: &Context, _: &Path) -> Response {
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
