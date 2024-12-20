/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use chrono::Utc;
use uuid::Uuid;

use crate::consts::DATABASE_PATH;
use crate::models::{User, UserRole};

pub fn open() -> Result<sqlite::Connection, sqlite::ConnectionError> {
    // Open database and create tables
    let database = sqlite::Connection::open(DATABASE_PATH)?;
    database.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id BLOB PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            email TEXT UNIQUE NOT NULL,
            password TEXT NOT NULL,
            role INTEGER NOT NULL,
            created_at TIMESTAMP NOT NULL,
            updated_at TIMESTAMP NOT NULL
        )",
        (),
    );
    database.execute(
        "CREATE TABLE IF NOT EXISTS sessions (
            id BLOB PRIMARY KEY,
            user_id BLOB NOT NULL,
            token TEXT UNIQUE NOT NULL,
            ip_address TEXT NOT NULL,
            ip_latitude REAL NULL,
            ip_longitude REAL NULL,
            ip_country TEXT NULL,
            ip_city TEXT NULL,
            client_name TEXT NULL,
            client_version TEXT NULL,
            client_os TEXT NULL,
            expires_at TIMESTAMP NOT NULL,
            created_at TIMESTAMP NOT NULL,
            updated_at TIMESTAMP NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )",
        (),
    );
    database.execute(
        "CREATE TABLE IF NOT EXISTS posts (
            id BLOB PRIMARY KEY,
            type INTEGER NOT NULL,
            parent_post_id BLOB NULL,
            user_id BLOB NOT NULL,
            text TEXT NOT NULL,
            replies INTEGER NOT NULL,
            reposts INTEGER NOT NULL,
            likes INTEGER NOT NULL,
            dislikes INTEGER NOT NULL,
            views INTEGER NOT NULL,
            created_at TIMESTAMP NOT NULL,
            updated_at TIMESTAMP NOT NULL,
            FOREIGN KEY (parent_post_id) REFERENCES posts(id) ON DELETE CASCADE,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )",
        (),
    );
    database.execute(
        "CREATE TABLE IF NOT EXISTS post_interactions (
            id BLOB PRIMARY KEY,
            post_id BLOB NOT NULL,
            user_id BLOB NOT NULL,
            type INTEGER NOT NULL,
            created_at TIMESTAMP NOT NULL,
            updated_at TIMESTAMP NOT NULL,
            FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )",
        (),
    );

    // Seed database
    let users_count = database
        .query::<i64>("SELECT COUNT(id) FROM users", ())
        .next()
        .expect("Should be some");
    if users_count == 0 {
        let now = Utc::now();
        let admin = User {
            id: Uuid::now_v7(),
            username: "admin".to_string(),
            email: "admin@plaatsoft.nl".to_string(),
            password: bcrypt::hash("admin", bcrypt::DEFAULT_COST).expect("Can't hash password"),
            role: UserRole::Admin,
            created_at: now,
            updated_at: now,
        };
        database.execute(
            format!(
                "INSERT INTO users ({}) VALUES ({})",
                User::columns(),
                User::values()
            ),
            admin,
        );
    }

    Ok(database)
}
