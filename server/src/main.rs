/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use std::net::{Ipv4Addr, TcpListener};
use std::sync::Arc;

use http::{Method, Response};
use router::Router;

use crate::consts::HTTP_PORT;
use crate::controllers::auth::{auth_login, auth_logout, auth_validate};
use crate::controllers::posts::{
    posts_create, posts_delete, posts_index, posts_show, posts_update,
};
use crate::controllers::search::search;
use crate::controllers::sessions::{sessions_index, sessions_revoke, sessions_show};
use crate::controllers::users::{
    users_change_password, users_create, users_index, users_posts, users_sessions, users_show,
    users_update,
};
use crate::controllers::{home, not_found};
use crate::models::{Session, User};

mod consts;
mod controllers;
mod database;
mod models;

#[derive(Clone)]
struct Context {
    database: sqlite::Connection,
    auth_user: Option<User>,
    auth_session: Option<Session>,
}

fn main() {
    let ctx = Context {
        database: database::open().expect("Can't open database"),
        auth_user: None,
        auth_session: None,
    };

    let router = Arc::new(
        Router::<Context>::new()
            .get("/", home)
            // Auth
            .post("/auth/login", auth_login)
            .get("/auth/validate", auth_validate)
            .put("/auth/logout", auth_logout)
            // Posts
            .get("/posts", posts_index)
            .post("/posts", posts_create)
            .get("/posts/:post_id", posts_show)
            .put("/posts/:post_id", posts_update)
            .delete("/posts/:post_id", posts_delete)
            // Search
            .get("/search", search)
            // Users
            .get("/users", users_index)
            .post("/users", users_create)
            .get("/users/:user_id", users_show)
            .put("/users/:user_id", users_update)
            .put("/users/:user_id/change_password", users_change_password)
            .get("/users/:user_id/sessions", users_sessions)
            .get("/users/:user_id/posts", users_posts)
            // Sessions
            .get("/sessions", sessions_index)
            .get("/sessions/:session_id", sessions_show)
            .delete("/sessions/:session_id", sessions_revoke)
            // Not found
            .fallback(not_found),
    );

    println!("Server is listening on: http://localhost:{}/", HTTP_PORT);
    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, HTTP_PORT))
        .unwrap_or_else(|_| panic!("Can't bind to port: {}", HTTP_PORT));
    http::serve(listener, move |req| {
        println!("{} {}", req.method, req.url.path);

        let mut ctx = ctx.clone();

        // Cors middleware
        if req.method == Method::Options {
            return Response::new()
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE")
                .header("Access-Control-Allow-Headers", "Authorization")
                .header("Access-Control-Max-Age", "86400");
        }

        // Auth middleware
        if !(
            req.url.path == "/"
                || req.url.path == "/auth/login"
                || (req.url.path == "/users" && req.method == Method::Post)
                || (req.url.path == "/posts" && req.method == Method::Get)
                || (req.url.path == "/search" && req.method == Method::Get)
            // FIXME: Posts show
            // FIXME: Users show
        ) {
            // Get token from Authorization header
            let authorization = match req
                .headers
                .get("Authorization")
                .or(req.headers.get("authorization"))
            {
                Some(authorization) => authorization,
                None => {
                    return Response::new()
                        .status(http::Status::Unauthorized)
                        .body("401 Unauthorized")
                        // Cors middleware
                        .header("Access-Control-Allow-Origin", "*")
                        .header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE")
                        .header("Access-Control-Allow-Headers", "Authorization")
                        .header("Access-Control-Max-Age", "86400");
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
            if session.is_none() {
                return Response::new()
                    .status(http::Status::Unauthorized)
                    .body("401 Unauthorized")
                    // Cors middleware
                    .header("Access-Control-Allow-Origin", "*")
                    .header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE")
                    .header("Access-Control-Allow-Headers", "Authorization")
                    .header("Access-Control-Max-Age", "86400");
            }
            let session = session.unwrap();

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
        }

        // Router
        let res = router.next(req, &ctx);

        // Cors middleware
        res.header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE")
            .header("Access-Control-Allow-Headers", "Authorization")
            .header("Access-Control-Max-Age", "86400")
    });
}
