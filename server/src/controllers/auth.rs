/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use std::collections::BTreeMap;
use std::fs;
use std::net::SocketAddr;

use axum::extract::{ConnectInfo, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::{Form, Json};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use serde::Deserialize;
use serde_json::json;
use sha2::Sha256;
use sqlx::{Pool, Sqlite};

use crate::consts;
use crate::models::session::Session;
use crate::models::user::User;

#[derive(Deserialize)]
pub struct LoginParams {
    logon: String,
    password: String,
}

#[derive(Deserialize)]
struct IpInfo {
    pub city: String,
    pub country: String,
    pub loc: String,
}

pub async fn auth_login(
    State(database): State<Pool<Sqlite>>,
    ConnectInfo(client_address): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Form(params): Form<LoginParams>,
) -> impl IntoResponse {
    // Find user by username or email
    let user = sqlx::query_as!(
        User,
        "SELECT id as \"id: uuid::Uuid\", username, email, password, created_at, updated_at FROM users WHERE username = ? OR email = ? LIMIT 1",
        params.logon,
        params.logon
    )
    .fetch_optional(&database)
    .await
    .unwrap();

    if let Some(user) = user {
        // Check password
        if !bcrypt::verify(&params.password, &user.password).unwrap() {
            return (
                StatusCode::UNAUTHORIZED,
                "Wrong username, email address or password",
            )
                .into_response();
        }

        // Get IP information from ipinfo.io
        let ip_info = reqwest::get(format!("https://ipinfo.io/{}/json", client_address.ip()))
            .await
            .unwrap()
            .json::<IpInfo>()
            .await
            .ok();

        // Parse user agent info
        let user_agent_parser = woothee::parser::Parser::new();
        let user_agent = headers
            .get("User-Agent")
            .and_then(|ua| user_agent_parser.parse(ua.to_str().unwrap()));

        // Create new session
        let session_id = uuid::Uuid::now_v7();
        let session = Session {
            id: session_id,
            user_id: user.id,
            ip_address: client_address.ip().to_string(),
            ip_latitude: ip_info
                .as_ref()
                .and_then(|info| info.loc.split(',').next().map(|lat| lat.parse().unwrap())),
            ip_longitude: ip_info
                .as_ref()
                .and_then(|info| info.loc.split(',').nth(1).map(|lon| lon.parse().unwrap())),
            ip_country: ip_info.as_ref().map(|info| info.country.clone()),
            ip_city: ip_info.as_ref().map(|info| info.city.clone()),
            client_name: user_agent.as_ref().map(|ua| ua.name.to_string()),
            client_version: user_agent.as_ref().map(|ua| ua.version.to_string()),
            client_os: user_agent.as_ref().map(|ua| ua.os.to_string()),
            expires_at: chrono::Utc::now().naive_utc() + consts::SESSION_EXPIRE_DURATION,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        };
        sqlx::query!(
            "INSERT INTO sessions (id, user_id, ip_address, ip_latitude, ip_longitude, ip_country, ip_city, client_name, client_version, client_os, expires_at, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            session.id,
            session.user_id,
            session.ip_address,
            session.ip_latitude,
            session.ip_longitude,
            session.ip_country,
            session.ip_city,
            session.client_name,
            session.client_version,
            session.client_os,
            session.expires_at,
            session.created_at,
            session.updated_at,
        ) .fetch_optional(&database)
        .await
        .unwrap();

        // Create jwt token
        let key_file = fs::read_to_string("jwt-secret.key").unwrap();
        let key = Hmac::<Sha256>::new_from_slice(key_file.as_bytes())
            .expect("HMAC can take key of any size");
        let mut claims = BTreeMap::new();
        claims.insert("user_id", user.id.to_string());
        claims.insert("session_id", session_id.to_string());
        Json(json!({
            "token": claims.sign_with_key(&key).unwrap(),
        }))
        .into_response()
    } else {
        (
            StatusCode::UNAUTHORIZED,
            "Wrong username, email address or password",
        )
            .into_response()
    }
}

pub async fn auth_logout() -> impl IntoResponse {
    // FIXME
    concat!("PlaatBook v", env!("CARGO_PKG_VERSION"))
}
