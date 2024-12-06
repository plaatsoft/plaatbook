/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use anyhow::Result;
use chrono::Utc;
use uuid::Uuid;

use crate::consts::DATABASE_PATH;
use crate::models::User;

pub fn open() -> Result<sqlite::Connection> {
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
    )?;

    // Seed database
    let users_count = database
        .query::<i64>("SELECT COUNT(id) FROM users", ())?
        .next()
        .unwrap()?;
    if users_count == 0 {
        let admin = User {
            id: Uuid::now_v7(),
            username: "admin".to_string(),
            email: "admin@plaatsoft.nl".to_string(),
            password: bcrypt::hash("admin", bcrypt::DEFAULT_COST)?,
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