/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use pbkdf2::password_verify;
use serde::Serialize;
use sqlite::{FromRow, FromValue};
use time::{Date, DateTime};
use uuid::Uuid;

use crate::{api, Context};

// MARK: User
#[derive(Clone, Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String, // FIXME: Hide in non admin / own user responses
    #[serde(skip)]
    pub password: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub birthdate: Option<Date>,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
    #[serde(skip)]
    pub role: UserRole,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Default for User {
    fn default() -> Self {
        let now = DateTime::now();
        User {
            id: Uuid::now_v7(),
            username: "".to_string(),
            email: "".to_string(),
            password: "".to_string(),
            firstname: None,
            lastname: None,
            birthdate: None,
            bio: None,
            location: None,
            website: None,
            role: UserRole::Normal,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Clone, Copy, Serialize, FromValue, Eq, PartialEq)]
pub enum UserRole {
    Normal = 0,
    Admin = 1,
}

impl From<User> for api::User {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            firstname: user.firstname,
            lastname: user.lastname,
            birthdate: user.birthdate.map(|date| date.to_string()),
            bio: user.bio,
            location: user.location,
            website: user.website,
            created_at: user.created_at,
            updated_at: user.updated_at,
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
        return Err(validate::Error::new("not unique"));
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
        return Err(validate::Error::new("not unique"));
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
    if !password_verify(value, &user.password).expect("Can't verify password") {
        return Err(validate::Error::new("incorrect"));
    }
    Ok(())
}
