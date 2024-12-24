/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use chrono::{NaiveDate, Utc};
use http::{Request, Response, Status};
use router::Path;
use serde::{Deserialize, Deserializer};
use uuid::Uuid;
use validate::Validate;

use crate::consts::LIMIT_DEFAULT;
use crate::controllers::not_found;
use crate::models::user::{
    is_current_password, is_unique_email, is_unique_email_or_auth_user_email, is_unique_username,
    is_unique_username_or_auth_user_username,
};
use crate::models::{IndexQuery, Post, Session, User, UserRole};
use crate::Context;

// MARK: Helpers
fn find_user(ctx: &Context, path: &Path) -> Option<User> {
    let user_id = path.get("user_id").expect("Should be some");
    let parsed_user_id = match user_id.parse::<Uuid>() {
        Ok(id) => id,
        Err(_) => Uuid::nil(),
    };

    ctx.database
        .query::<User>(
            format!(
                "SELECT {} FROM users WHERE id = ? OR username = ? LIMIT 1",
                User::columns()
            ),
            (parsed_user_id, user_id.clone()),
        )
        .next()
}

// MARK: Users index
pub fn users_index(req: &Request, ctx: &Context, _: &Path) -> Response {
    // Authorization
    let auth_user = ctx.auth_user.as_ref().expect("Not authed");
    if !(auth_user.role == UserRole::Admin) {
        return Response::new()
            .status(Status::Unauthorized)
            .body("401 Unauthorized");
    }

    // Parse request query
    let query = match req.url.query.as_ref() {
        Some(query) => match serde_urlencoded::from_str::<IndexQuery>(query) {
            Ok(query) => query,
            Err(_) => return Response::with_status(Status::BadRequest),
        },
        None => IndexQuery::default(),
    };
    if let Err(report) = query.validate() {
        return Response::with_status(Status::BadRequest).json(report);
    }

    // Get users
    let limit = query.limit.unwrap_or(LIMIT_DEFAULT);
    let users = ctx
        .database
        .query::<User>(
            format!(
                "SELECT {} FROM users WHERE username LIKE ? LIMIT ? OFFSET ?",
                User::columns()
            ),
            (
                format!("%{}%", query.query.unwrap_or_default().replace("%", "\\%")),
                limit,
                limit * (query.page.unwrap_or(1) - 1),
            ),
        )
        .collect::<Vec<_>>();
    Response::new().json(users)
}

// MARK: Users create
pub fn users_create(req: &Request, ctx: &Context, _: &Path) -> Response {
    // Authorization
    // -

    // Parse and validate body
    #[derive(Deserialize, Validate)]
    #[validate(context(Context))]
    struct Body {
        #[validate(ascii, length(min = 1, max = 32), custom(is_unique_username))]
        username: String,
        #[validate(email, length(max = 128), custom(is_unique_email))]
        email: String,
        #[validate(ascii, length(min = 6, max = 128))]
        password: String,
    }
    let body = match serde_urlencoded::from_str::<Body>(&req.body) {
        Ok(body) => body,
        Err(_) => {
            return Response::new()
                .status(Status::BadRequest)
                .body("400 Bad Request");
        }
    };
    if let Err(errors) = body.validate_with(ctx) {
        return Response::new().status(Status::BadRequest).json(errors);
    }

    // Create new user
    let user = User {
        username: body.username,
        email: body.email,
        password: bcrypt::hash(body.password, bcrypt::DEFAULT_COST).expect("Can't hash password"),
        role: UserRole::Normal,
        ..Default::default()
    };
    ctx.database.execute(
        format!(
            "INSERT INTO users ({}) VALUES ({})",
            User::columns(),
            User::values()
        ),
        user.clone(),
    );

    Response::new().json(user)
}

// MARK: Users show
pub fn users_show(req: &Request, ctx: &Context, path: &Path) -> Response {
    let user = match find_user(ctx, path) {
        Some(user) => user,
        None => return not_found(req, ctx, path),
    };

    // Authorization
    // -

    Response::new().json(user)
}

// MARK: Users update

fn empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(s) if s.is_empty() => Ok(None),
        _ => Ok(opt),
    }
}

pub fn users_update(req: &Request, ctx: &Context, path: &Path) -> Response {
    let mut user = match find_user(ctx, path) {
        Some(user) => user,
        None => return not_found(req, ctx, path),
    };

    // Authorization
    let auth_user = ctx.auth_user.as_ref().expect("Not authed");
    if !(user.id == auth_user.id || auth_user.role == UserRole::Admin) {
        return Response::new()
            .status(Status::Unauthorized)
            .body("401 Unauthorized");
    }

    // Parse and validate body
    #[derive(Deserialize, Validate)]
    #[validate(context(Context))]
    struct Body {
        #[validate(
            ascii,
            length(min = 1, max = 32),
            custom(is_unique_username_or_auth_user_username)
        )]
        username: String,
        #[validate(email, length(max = 128), custom(is_unique_email_or_auth_user_email))]
        email: String,
        #[serde(deserialize_with = "empty_string_as_none")]
        #[validate(length(max = 64))]
        firstname: Option<String>,
        #[serde(deserialize_with = "empty_string_as_none")]
        #[validate(length(max = 64))]
        lastname: Option<String>,
        #[serde(deserialize_with = "empty_string_as_none")]
        birthdate: Option<String>,
        #[serde(deserialize_with = "empty_string_as_none")]
        #[validate(length(max = 512))]
        bio: Option<String>,
        #[serde(deserialize_with = "empty_string_as_none")]
        #[validate(length(max = 128))]
        location: Option<String>,
        #[serde(deserialize_with = "empty_string_as_none")]
        #[validate(url, length(max = 512))]
        website: Option<String>,
    }
    let body = match serde_urlencoded::from_str::<Body>(&req.body) {
        Ok(body) => body,
        Err(e) => {
            println!("{:?}", e);
            return Response::new()
                .status(Status::BadRequest)
                .body("400 Bad Request");
        }
    };
    if let Err(errors) = body.validate_with(ctx) {
        return Response::new().status(Status::BadRequest).json(errors);
    }

    // Update user
    user.username = body.username;
    user.email = body.email;
    user.firstname = body.firstname;
    user.lastname = body.lastname;
    user.birthdate = body
        .birthdate
        .and_then(|birthdate| NaiveDate::parse_from_str(&birthdate, "%Y-%m-%d").ok());
    user.bio = body.bio;
    user.location = body.location;
    user.website = body.website;
    user.updated_at = Utc::now();
    ctx.database.execute(
        "UPDATE users SET username = ?, email = ?, firstname = ?, lastname = ?, birthdate = ?, bio = ?, location = ?, website = ?, updated_at = ? WHERE id = ?",
        (
            user.username.clone(),
            user.email.clone(),
            user.firstname.clone(),
            user.lastname.clone(),
            user.birthdate,
            user.bio.clone(),
            user.location.clone(),
            user.website.clone(),
            user.updated_at,
            user.id,
        ),
    );

    Response::new().json(user)
}

// MARK: Users change password
pub fn users_change_password(req: &Request, ctx: &Context, path: &Path) -> Response {
    let mut user = match find_user(ctx, path) {
        Some(user) => user,
        None => return not_found(req, ctx, path),
    };

    // Authorization
    let auth_user = ctx.auth_user.as_ref().expect("Not authed");
    if !(user.id == auth_user.id || auth_user.role == UserRole::Admin) {
        return Response::new()
            .status(Status::Unauthorized)
            .body("401 Unauthorized");
    }

    // Parse and validate body
    #[derive(Deserialize, Validate)]
    #[validate(context(Context))]
    struct Body {
        #[validate(ascii, custom(is_current_password))]
        current_password: String,
        #[validate(ascii, length(min = 6, max = 128))]
        password: String,
    }
    let body = match serde_urlencoded::from_str::<Body>(&req.body) {
        Ok(body) => body,
        Err(_) => {
            return Response::new()
                .status(Status::BadRequest)
                .body("400 Bad Request");
        }
    };
    if let Err(errors) = body.validate_with(ctx) {
        return Response::new().status(Status::BadRequest).json(errors);
    }

    // Update user
    user.password = bcrypt::hash(body.password, bcrypt::DEFAULT_COST).expect("Can't hash password");
    user.updated_at = Utc::now();
    ctx.database.execute(
        "UPDATE users SET password = ?, updated_at = ? WHERE id = ?",
        (user.password.clone(), user.updated_at, user.id),
    );

    Response::new().json(user)
}

// MARK: Users sessions
pub fn users_sessions(req: &Request, ctx: &Context, path: &Path) -> Response {
    let user = match find_user(ctx, path) {
        Some(user) => user,
        None => return not_found(req, ctx, path),
    };

    // Authorization
    let auth_user = ctx.auth_user.as_ref().expect("Not authed");
    if !(user.id == auth_user.id || auth_user.role == UserRole::Admin) {
        return Response::new()
            .status(Status::Unauthorized)
            .body("401 Unauthorized");
    }

    let user_sessions = ctx
            .database
            .query::<Session>(
                format!(
                    "SELECT {} FROM sessions WHERE user_id = ? AND expires_at > ? ORDER BY expires_at DESC",
                    Session::columns()
                ),
                (user.id, Utc::now()),
            )
            .collect::<Vec<_>>();
    Response::new().json(user_sessions)
}

// MARK: Users posts
pub fn users_posts(req: &Request, ctx: &Context, path: &Path) -> Response {
    let user = match find_user(ctx, path) {
        Some(user) => user,
        None => return not_found(req, ctx, path),
    };

    // Authorization
    // -

    // Parse request query
    let query = match req.url.query.as_ref() {
        Some(query) => match serde_urlencoded::from_str::<IndexQuery>(query) {
            Ok(query) => query,
            Err(_) => return Response::with_status(Status::BadRequest),
        },
        None => IndexQuery::default(),
    };
    if let Err(report) = query.validate() {
        return Response::with_status(Status::BadRequest).json(report);
    }

    // Get user posts
    let limit = query.limit.unwrap_or(LIMIT_DEFAULT);
    let user_posts = ctx
        .database
        .query::<Post>(
            format!(
                "SELECT {} FROM posts WHERE user_id = ? AND text LIKE ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
                Post::columns()
            ),
            (
                user.id,
                format!("%{}%", query.query.unwrap_or_default().replace("%", "\\%")),
                limit,
                limit * (query.page.unwrap_or(1) - 1),
            )
        )
        .map(|mut post| {
            post.process(ctx);
            post
        })
        .collect::<Vec<_>>();
    Response::new().json(user_posts)
}
