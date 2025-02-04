/*
 * Copyright (c) 2024-2025 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use std::net::{Ipv4Addr, TcpListener};
use std::path::Path;
use std::sync::LazyLock;

use router::{Router, RouterBuilder};
use simple_useragent::UserAgentParser;

use crate::controllers::auth::{auth_login, auth_logout, auth_validate};
use crate::controllers::posts::{
    posts_create, posts_create_reply, posts_delete, posts_dislike, posts_dislike_delete,
    posts_index, posts_like, posts_like_delete, posts_replies, posts_repost, posts_show,
    posts_update,
};
use crate::controllers::sessions::{sessions_index, sessions_revoke, sessions_show};
use crate::controllers::users::{
    users_change_password, users_create, users_index, users_posts, users_sessions, users_show,
    users_update,
};
use crate::controllers::{home, not_found};
use crate::layers::{
    auth_optional_pre_layer, auth_required_pre_layer, cors_post_layer, cors_pre_layer,
    log_pre_layer,
};
use crate::models::{Session, User};

mod api {
    include!(concat!(env!("OUT_DIR"), "/api.rs"));
}
mod controllers;
mod database;
mod layers;
mod models;
#[cfg(test)]
mod test_utils;

// MARK: Context
static USER_AGENT_PARSER: LazyLock<UserAgentParser> = LazyLock::new(UserAgentParser::new);

#[derive(Clone)]
pub(crate) struct Context {
    database: sqlite::Connection,
    auth_user: Option<User>,
    auth_session: Option<Session>,
}

impl Context {
    pub(crate) fn with_database(database_path: impl AsRef<Path>) -> Self {
        let database = database::open(database_path.as_ref()).expect("Can't open database");
        database::seed(&database);
        Self {
            database,
            auth_user: None,
            auth_session: None,
        }
    }

    #[cfg(test)]
    pub(crate) fn with_test_database() -> Self {
        let database = database::open(Path::new(":memory:")).expect("Can't open database");
        Self {
            database,
            auth_user: None,
            auth_session: None,
        }
    }
}

// MARK: Router
pub(crate) fn router(ctx: Context) -> Router<Context> {
    // Guests routes
    let router = RouterBuilder::<Context>::with(ctx)
        .pre_layer(log_pre_layer)
        .pre_layer(cors_pre_layer)
        .post_layer(cors_post_layer)
        .pre_layer(auth_optional_pre_layer)
        .get("/", home)
        // Auth
        .post("/auth/login", auth_login)
        // Posts
        .get("/posts", posts_index)
        .get("/posts/:post_id", posts_show)
        .get("/posts/:post_id/replies", posts_replies)
        // Users
        .post("/users", users_create)
        .get("/users/:user_id", users_show)
        .get("/users/:user_id/posts", users_posts)
        // Not found
        .fallback(not_found);

    // Authed routes
    router
        .pre_layer(auth_required_pre_layer)
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
        .build()
}

// MARK: Main
fn main() {
    println!("Starting PlaatBook server...");

    // Init database and user agent parser
    let router = router(Context::with_database("database.db"));
    let _ = &*USER_AGENT_PARSER;

    // Start server
    const HTTP_PORT: u16 = 8080;
    println!("Server is listening on: http://localhost:{}/", HTTP_PORT);
    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, HTTP_PORT))
        .unwrap_or_else(|_| panic!("Can't bind to port: {}", HTTP_PORT));
    http::serve(listener, move |req| router.handle(req));
}
