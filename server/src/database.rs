/*
 * Copyright (c) 2024-2025 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use chrono::NaiveDate;
use const_format::formatcp;
use pbkdf2::password_hash;

use crate::models::{Post, Session, User, UserRole};

// MARK: Database extension
pub trait Extension {
    fn insert_user(&self, user: User);
    fn insert_session(&self, session: Session);
    fn insert_post(&self, post: Post);
}

impl Extension for bsqlite::Connection {
    fn insert_user(&self, user: User) {
        self.execute(
            formatcp!(
                "INSERT INTO users ({}) VALUES ({})",
                User::columns(),
                User::values()
            ),
            user,
        );
    }

    fn insert_session(&self, session: Session) {
        self.execute(
            formatcp!(
                "INSERT INTO sessions ({}) VALUES ({})",
                Session::columns(),
                Session::values()
            ),
            session,
        );
    }

    fn insert_post(&self, post: Post) {
        self.execute(
            formatcp!(
                "INSERT INTO posts ({}) VALUES ({})",
                Post::columns(),
                Post::values()
            ),
            post,
        );
    }
}

// MARK: Create tables
pub fn create_tables(database: &bsqlite::Connection) {
    database.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id BLOB PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            email TEXT UNIQUE NOT NULL,
            password TEXT NOT NULL,
            firstname TEXT NULL,
            lastname TEXT NULL,
            birthdate INTEGER NULL,
            bio TEXT NULL,
            location TEXT NULL,
            website TEXT NULL,
            role INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
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
            expires_at INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
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
            text TEXT NULL,
            replies INTEGER NOT NULL,
            reposts INTEGER NOT NULL,
            likes INTEGER NOT NULL,
            dislikes INTEGER NOT NULL,
            views INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
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
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )",
        (),
    );
}

// MARK: Seed database
pub fn seed(database: &bsqlite::Connection) {
    let users_count = database
        .query::<i64>("SELECT COUNT(id) FROM users", ())
        .next()
        .expect("Should be some");
    if users_count == 0 {
        database.insert_user(User {
            username: "admin".to_string(),
            email: "admin@plaatsoft.nl".to_string(),
            password: password_hash("admin"),
            firstname: Some("Admin".to_string()),
            birthdate: NaiveDate::from_ymd_opt(2024, 12, 2),
            bio: Some("Admin of PlaatBook".to_string()),
            location: Some("Gouda, Netherlands".to_string()),
            website: Some("https://www.plaatsoft.nl/".to_string()),
            role: UserRole::Admin,
            ..Default::default()
        });
    }
}
