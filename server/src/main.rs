/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use std::net::{Ipv4Addr, TcpListener};

use router::Router;

use crate::consts::HTTP_PORT;
use crate::controllers::auth::{auth_login, auth_logout, auth_validate};
use crate::controllers::posts::{
    posts_create, posts_create_reply, posts_delete, posts_dislike, posts_dislike_delete,
    posts_index, posts_like, posts_like_delete, posts_replies, posts_repost, posts_show,
    posts_update,
};
use crate::controllers::search::search;
use crate::controllers::sessions::{sessions_index, sessions_revoke, sessions_show};
use crate::controllers::users::{
    users_change_password, users_create, users_index, users_posts, users_sessions, users_show,
    users_update,
};
use crate::controllers::{home, not_found};
use crate::layers::{
    auth_optional_layer, auth_required_layer, cors_post_layer, cors_pre_layer, log_layer,
};
use crate::models::{Session, User};

mod consts;
mod controllers;
mod database;
mod layers;
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

    // Guest routes
    let router = Router::<Context>::with(ctx)
        .pre_layer(log_layer)
        .pre_layer(cors_pre_layer)
        .post_layer(cors_post_layer)
        .pre_layer(auth_optional_layer)
        .get("/", home)
        // Auth
        .post("/auth/login", auth_login)
        // Posts
        .get("/posts", posts_index)
        .get("/posts/:post_id", posts_show)
        .get("/posts/:post_id/replies", posts_replies)
        // Search
        .get("/search", search)
        // Users
        .post("/users", users_create)
        .get("/users/:user_id", users_show)
        .get("/users/:user_id/posts", users_posts)
        // Not found
        .fallback(not_found);

    // Authed routes
    let router = router
        .pre_layer(auth_required_layer)
        // Auth
        .get("/auth/validate", auth_validate)
        .put("/auth/logout", auth_logout)
        // Posts
        .post("/posts", posts_create)
        .put("/posts/:post_id", posts_update)
        .delete("/posts/:post_id", posts_delete)
        .post("/posts/:post_id/reply", posts_create_reply)
        .post("/posts/:post_id/repost", posts_repost)
        .put("/posts/:post_id/like", posts_like)
        .delete("/posts/:post_id/like", posts_like_delete)
        .put("/posts/:post_id/dislike", posts_dislike)
        .delete("/posts/:post_id/dislike", posts_dislike_delete)
        // Users
        .get("/users", users_index)
        .put("/users/:user_id", users_update)
        .put("/users/:user_id/change_password", users_change_password)
        .get("/users/:user_id/sessions", users_sessions)
        // Sessions
        .get("/sessions", sessions_index)
        .get("/sessions/:session_id", sessions_show)
        .delete("/sessions/:session_id", sessions_revoke)
        .build();

    println!("Server is listening on: http://localhost:{}/", HTTP_PORT);
    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, HTTP_PORT))
        .unwrap_or_else(|_| panic!("Can't bind to port: {}", HTTP_PORT));
    http::serve(listener, move |req| router.handle(req));
}
