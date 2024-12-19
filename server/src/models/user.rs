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

// MARK: User
#[derive(Clone, Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip)]
    pub password: String,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// MARK: User role
#[derive(Clone, Copy, Serialize, Eq, PartialEq)]
pub enum UserRole {
    Normal = 0,
    Admin = 1,
}
impl From<UserRole> for sqlite::Value {
    fn from(value: UserRole) -> Self {
        sqlite::Value::Integer(value as i64)
    }
}
impl TryFrom<sqlite::Value> for UserRole {
    type Error = sqlite::ValueError;
    fn try_from(value: sqlite::Value) -> Result<Self, Self::Error> {
        match value {
            sqlite::Value::Integer(0) => Ok(UserRole::Normal),
            sqlite::Value::Integer(1) => Ok(UserRole::Admin),
            _ => Err(sqlite::ValueError),
        }
    }
}

// MARK: Validators
pub fn is_unique_username(value: &str, context: &Context) -> validate::Result {
    let count = context
        .database
        .query::<i64>(
            "SELECT COUNT(id) FROM users WHERE username = ?",
            value.to_string(),
        )
        .next()
        .expect("Should be some");
    if count != 0 {
        return Err(validate::Error::new("username is not unique"));
    }
    Ok(())
}

pub fn is_unique_username_or_auth_user_username(
    value: &str,
    context: &Context,
) -> validate::Result {
    if value == context.auth_user.as_ref().expect("Not authed").username {
        return Ok(());
    }
    is_unique_username(value, context)
}

pub fn is_unique_email(value: &str, context: &Context) -> validate::Result {
    let count = context
        .database
        .query::<i64>(
            "SELECT COUNT(id) FROM users WHERE email = ?",
            value.to_string(),
        )
        .next()
        .expect("Should be some");
    if count != 0 {
        return Err(validate::Error::new("email is not unique"));
    }
    Ok(())
}

pub fn is_unique_email_or_auth_user_email(value: &str, context: &Context) -> validate::Result {
    if value == context.auth_user.as_ref().expect("Not authed").email {
        return Ok(());
    }
    is_unique_email(value, context)
}

pub fn is_current_password(value: &str, context: &Context) -> validate::Result {
    let user = context.auth_user.as_ref().expect("Not authed");
    if !bcrypt::verify(value, &user.password).expect("Can't verify password") {
        return Err(validate::Error::new("password is incorrect"));
    }
    Ok(())
}
