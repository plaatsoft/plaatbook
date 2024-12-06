/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlite::FromRow;
use uuid::Uuid;

use crate::Context;

#[derive(Clone, Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip)]
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub fn is_unique_username(value: &str, context: &Context) -> garde::Result {
    let count = context
        .database
        .query::<i64>(
            "SELECT COUNT(*) FROM users WHERE username = ?",
            value.to_string(),
        )
        .unwrap()
        .next()
        .unwrap()
        .unwrap();
    if count != 0 {
        return Err(garde::Error::new("username is not unique"));
    }
    Ok(())
}

pub fn is_unique_email(value: &str, context: &Context) -> garde::Result {
    let count = context
        .database
        .query::<i64>(
            "SELECT COUNT(*) FROM users WHERE email = ?",
            value.to_string(),
        )
        .unwrap()
        .next()
        .unwrap()
        .unwrap();
    if count != 0 {
        return Err(garde::Error::new("email is not unique"));
    }
    Ok(())
}
