/*
 * Copyright (c) 2024-2025 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use bsqlite::{FromRow, FromValue};
use chrono::{DateTime, NaiveDate, Utc};
use pbkdf2::password_verify;
use uuid::Uuid;

use crate::{api, Context};

// MARK: User
#[derive(Clone, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub birthdate: Option<NaiveDate>,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for User {
    fn default() -> Self {
        let now = Utc::now();
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

#[derive(Clone, Copy, FromValue, Eq, PartialEq)]
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

pub fn is_auth_user_current_password(value: &str, context: &Context) -> validate::Result {
    let user = context.auth_user.as_ref().expect("Not authed");
    if !password_verify(value, &user.password).expect("Can't verify password") {
        return Err(validate::Error::new("incorrect"));
    }
    Ok(())
}

// MARK: Tests
#[cfg(test)]
mod test {
    use pbkdf2::password_hash;

    use super::*;
    use crate::database::Extension;

    #[test]
    fn test_is_unique_username() {
        let ctx = Context::with_test_database();
        ctx.database.insert_user(User {
            username: "existing_username".to_string(),
            ..Default::default()
        });
        assert!(is_unique_username("unique_username", &ctx).is_ok());
        assert!(is_unique_username("existing_username", &ctx).is_err());
    }

    #[test]
    fn test_is_unique_username_or_auth_user_username() {
        let mut ctx = Context::with_test_database();
        ctx.database.insert_user(User {
            username: "existing_username".to_string(),
            ..Default::default()
        });
        ctx.auth_user = Some(User {
            username: "auth_username".to_string(),
            ..Default::default()
        });
        assert!(is_unique_username_or_auth_user_username("auth_username", &ctx).is_ok());
        assert!(is_unique_username_or_auth_user_username("unique_username", &ctx).is_ok());
        assert!(is_unique_username_or_auth_user_username("existing_username", &ctx).is_err());
    }

    #[test]
    fn test_is_unique_email() {
        let ctx = Context::with_test_database();
        ctx.database.insert_user(User {
            email: "existing_email@example.com".to_string(),
            ..Default::default()
        });
        assert!(is_unique_email("unique_email@example.com", &ctx).is_ok());
        assert!(is_unique_email("existing_email@example.com", &ctx).is_err());
    }

    #[test]
    fn test_is_unique_email_or_auth_user_email() {
        let mut ctx = Context::with_test_database();
        ctx.database.insert_user(User {
            email: "existing_email@example.com".to_string(),
            ..Default::default()
        });
        ctx.auth_user = Some(User {
            email: "auth_email@example.com".to_string(),
            ..Default::default()
        });
        assert!(is_unique_email_or_auth_user_email("auth_email@example.com", &ctx).is_ok());
        assert!(is_unique_email_or_auth_user_email("unique_email@example.com", &ctx).is_ok());
        assert!(is_unique_email_or_auth_user_email("existing_email@example.com", &ctx).is_err());
    }

    #[test]
    fn test_is_auth_user_current_password() {
        let mut ctx = Context::with_test_database();
        ctx.auth_user = Some(User {
            password: password_hash("correct_password"),
            ..Default::default()
        });
        assert!(is_auth_user_current_password("correct_password", &ctx).is_ok());
        assert!(is_auth_user_current_password("incorrect_password", &ctx).is_err());
    }
}
