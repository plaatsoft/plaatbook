/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use anyhow::Result;
use base64::prelude::*;
use chrono::Utc;
use http::{Request, Response, Status};
use router::Path;
use serde::Deserialize;
use serde_json::json;

use crate::consts::SESSION_EXPIRE_DURATION;
use crate::models::{Session, User};
use crate::Context;

pub fn auth_login(req: &Request, ctx: &Context, _: &Path) -> Result<Response> {
    // Parse body
    #[derive(Deserialize)]
    struct Body {
        logon: String,
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

    // Find user by username or email
    let user = ctx
        .database
        .query::<User>(
            format!(
                "SELECT {} FROM users WHERE username = ? OR email = ? LIMIT 1",
                User::columns()
            ),
            (body.logon.clone(), body.logon),
        )?
        .next();
    if user.is_none() {
        return Ok(Response::new()
            .status(Status::Unauthorized)
            .body("Wrong username, email address or password"));
    }

    // Check password
    let user = user.unwrap()?;
    if !bcrypt::verify(&body.password, &user.password)? {
        return Ok(Response::new()
            .status(Status::Unauthorized)
            .body("Wrong username, email address or password"));
    }

    // Get IP information from ipinfo.io
    #[derive(Deserialize)]
    struct IpInfo {
        city: String,
        country: String,
        loc: String,
    }
    let ip_res = http::fetch(Request::new().host("ipinfo.io").path("/json"))?;
    let ip_info = serde_json::from_str::<IpInfo>(&ip_res.body).ok();

    // Parse user agent info
    let user_agent_parser = woothee::parser::Parser::new();
    let user_agent = req
        .headers
        .get("User-Agent")
        .and_then(|user_agent| user_agent_parser.parse(user_agent));

    // Generate token
    let mut token_bytes = [0u8; 256];
    getrandom::getrandom(&mut token_bytes)?;
    let token = BASE64_STANDARD.encode(token_bytes);

    // Create new session
    let session = Session {
        id: uuid::Uuid::now_v7(),
        user_id: user.id,
        token,
        ip_address: req.client_addr.unwrap().ip().to_string(),
        ip_latitude: ip_info
            .as_ref()
            .and_then(|info| info.loc.split(',').next().unwrap().parse().ok()),
        ip_longitude: ip_info
            .as_ref()
            .and_then(|info| info.loc.split(',').nth(1).unwrap().parse().ok()),
        ip_country: ip_info.as_ref().map(|info| info.country.clone()),
        ip_city: ip_info.as_ref().map(|info| info.city.clone()),
        client_name: user_agent.as_ref().map(|ua| ua.name.to_string()),
        client_version: user_agent.as_ref().map(|ua| ua.version.to_string()),
        client_os: user_agent.as_ref().map(|ua| ua.os.to_string()),
        expires_at: Utc::now() + SESSION_EXPIRE_DURATION,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    ctx.database
        .query::<()>(
            format!(
                "INSERT INTO sessions ({}) VALUES ({})",
                Session::columns(),
                Session::params()
            ),
            session.clone(),
        )?
        .next();

    // Return session
    Ok(Response::new().json(json!({
        "token": session.token,
        "session": session,
        "user": user,
    })))
}

pub fn auth_validate(_: &Request, ctx: &Context, _: &Path) -> Result<Response> {
    Ok(Response::new().json(json!({
        "session": ctx.auth_session,
        "user": ctx.auth_user,
    })))
}

pub fn auth_logout(_: &Request, ctx: &Context, _: &Path) -> Result<Response> {
    // Expire session
    ctx.database
        .query::<()>(
            "UPDATE sessions SET expires_at = ? WHERE token = ?",
            (Utc::now(), ctx.auth_session.as_ref().unwrap().token.clone()),
        )?
        .next();
    Ok(Response::new().status(Status::Ok))
}
