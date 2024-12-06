/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use std::net::{Ipv4Addr, TcpListener};
use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use http::Response;
use router::Router;
use uuid::Uuid;

use crate::controllers::users::{users_index, users_show, users_store};
use crate::controllers::{home, not_found};
use crate::models::User;

mod controllers;
mod models;

const DATABASE_PATH: &str = "database.db";
const HTTP_PORT: u16 = 8000;

// MARK: Database
#[derive(Clone)]
struct Context {
    database: Arc<sqlite::Connection>,
}

fn open_database() -> Result<sqlite::Connection> {
    // Open database and create tables
    let database = sqlite::Connection::open(DATABASE_PATH)?;
    database.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id BLOB PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            email TEXT UNIQUE NOT NULL,
            password TEXT NOT NULL,
            created_at TIMESTAMP NOT NULL,
            updated_at TIMESTAMP NOT NULL
        )",
    )?;

    // Seed database
    let users_count = database
        .query::<i64>("SELECT COUNT(id) FROM users", ())?
        .next()
        .expect("Should be some")?;
    if users_count == 0 {
        let admin = User {
            id: Uuid::now_v7(),
            username: "admin".to_string(),
            email: "info@plaatsoft.nl".to_string(),
            password: "password".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        database
            .query::<()>(
                format!(
                    "INSERT INTO users ({}) VALUES ({})",
                    User::columns(),
                    User::params()
                ),
                admin,
            )?
            .next();
    }

    Ok(database)
}

// MARK: Main
fn main() {
    let ctx = Context {
        database: Arc::new(open_database().expect("Can't open database")),
    };

    let router = Arc::new(
        Router::<Context>::new()
            .get("/", home)
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
    });
}
