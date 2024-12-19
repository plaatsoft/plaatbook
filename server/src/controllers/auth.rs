/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use base64::prelude::*;
use chrono::Utc;
use http::{Request, Response, Status};
use router::Path;
use serde::Deserialize;
use serde_json::json;

use crate::models::{Session, User};
use crate::Context;

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
    if !bcrypt::verify(&body.password, &user.password).expect("Can't verify password") {
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
    let user_agent_parser = woothee::parser::Parser::new();
    let user_agent = req
        .headers
        .get("User-Agent")
        .and_then(|user_agent| user_agent_parser.parse(user_agent));

    // Generate token
    let mut token_bytes = [0u8; 256];
    getrandom::getrandom(&mut token_bytes).expect("Can't get random bytes");
    let token = BASE64_STANDARD.encode(token_bytes);

    // Create new session
    let session = Session {
        user_id: user.id,
        token,
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
        client_name: user_agent.as_ref().map(|ua| ua.name.to_string()),
        client_version: user_agent.as_ref().map(|ua| ua.version.to_string()),
        client_os: user_agent.as_ref().map(|ua| ua.os.to_string()),
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
    Response::new().json(json!({
        "token": session.token,
        "session": session,
        "user": user,
    }))
}

// MARK: Auth validate
pub fn auth_validate(_: &Request, ctx: &Context, _: &Path) -> Response {
    Response::new().json(json!({
        "session": ctx.auth_session,
        "user": ctx.auth_user,
    }))
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
