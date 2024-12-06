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
use crate::controllers::users::{users_index, users_show, users_store};
use crate::controllers::{home, not_found};
use crate::models::{Session, User};

mod consts;
mod controllers;
mod database;
mod models;

#[derive(Clone)]
struct Context {
    database: Arc<sqlite::Connection>,
    auth_user: Option<User>,
    auth_session: Option<Session>,
}

fn main() {
    let ctx = Context {
        database: Arc::new(database::open().expect("Can't open database")),
        auth_user: None,
        auth_session: None,
    };

    let router = Arc::new(
        Router::<Context>::new()
            .get("/", home)
            // Auth
            .post("/auth/login", auth_login)
            .get("/auth/validate", auth_validate)
            .post("/auth/logout", auth_logout)
            // Users
            .get("/users", users_index)
            .post("/users", users_store)
            .get("/users/:user_id", users_show)
            // Not found
            .fallback(not_found),
    );

    println!("Server is listening on: http://localhost:{}/", HTTP_PORT);
    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, HTTP_PORT))
        .unwrap_or_else(|_| panic!("Can't bind to port: {}", HTTP_PORT));
    http::serve(listener, move |req| {
        println!("{} {}", req.method, req.path);

        let mut ctx = ctx.clone();

        // Cors middleware
        if req.method == Method::Options {
            return Response::new()
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Headers", "Authorization");
        }

        // Auth middleware
        if !(req.path == "/"
            || req.path == "/auth/login"
            || (req.path == "/users" && req.method == Method::Post))
        {
            // Get token from Authorization header
            let authorization = match req.headers.get("Authorization") {
                Some(authorization) => authorization,
                None => {
                    return Response::new()
                        .status(http::Status::Unauthorized)
                        .body("401 Unauthorized");
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
                .unwrap()
                .next();
            if session.is_none() {
                return Response::new()
                    .status(http::Status::Unauthorized)
                    .body("401 Unauthorized");
            }
            let session = session.unwrap().unwrap();

            // Get user by session user_id
            ctx.auth_user = Some(
                ctx.database
                    .query::<models::User>(
                        format!(
                            "SELECT {} FROM users WHERE id = ? LIMIT 1",
                            models::User::columns()
                        ),
                        session.user_id,
                    )
                    .unwrap()
                    .next()
                    .unwrap()
                    .unwrap(),
            );
            ctx.auth_session = Some(session);
        }

        // Error middleware
        let res = match router.next(req, &ctx) {
            Ok(res) => res,
            Err(err) => {
                println!("Error: {:?}", err);
                Response::new()
                    .status(http::Status::InternalServerError)
                    .body("500 Internal Server Error")
            }
        };

        // Cors middleware
        res.header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Headers", "Authorization")
    });
}
