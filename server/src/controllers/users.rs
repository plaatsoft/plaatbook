/*
 * Copyright (c) 2024-2025 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use std::str::FromStr;

use chrono::{NaiveDate, Utc};
use const_format::formatcp;
use pbkdf2::password_hash;
use serde::{Deserialize, Deserializer};
use small_http::{Request, Response, Status};
use uuid::Uuid;
use validate::Validate;

use crate::controllers::not_found;
use crate::database::Extension;
use crate::models::user::{
    is_auth_user_current_password, is_unique_email, is_unique_email_or_auth_user_email,
    is_unique_username, is_unique_username_or_auth_user_username,
};
use crate::models::{IndexQuery, Post, Session, User, UserRole};
use crate::{api, Context};

// MARK: Helpers
fn find_user(req: &Request, ctx: &Context) -> Option<User> {
    let user_id = req.params.get("user_id").expect("Should be some");
    let parsed_user_id = match user_id.parse::<Uuid>() {
        Ok(id) => id,
        Err(_) => Uuid::nil(),
    };

    ctx.database
        .query::<User>(
            formatcp!(
                "SELECT {} FROM users WHERE id = ? OR username = ? LIMIT 1",
                User::columns()
            ),
            (parsed_user_id, user_id.clone()),
        )
        .next()
}

// MARK: Users index
pub fn users_index(req: &Request, ctx: &Context) -> Response {
    // Authorization
    let auth_user = ctx.auth_user.as_ref().expect("Not authed");
    if !(auth_user.role == UserRole::Admin) {
        return Response::new()
            .status(Status::Unauthorized)
            .body("401 Unauthorized");
    }

    // Parse request query
    let query = match req.url.query() {
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
    let search_query = format!("%{}%", query.query.replace("%", "\\%"));
    let total = ctx
        .database
        .query::<i64>(
            "SELECT COUNT(id) FROM users WHERE username LIKE ?",
            search_query.clone(),
        )
        .next()
        .expect("Can't count users");
    let users = ctx
        .database
        .query::<User>(
            formatcp!(
                "SELECT {} FROM users WHERE username LIKE ? LIMIT ? OFFSET ?",
                User::columns()
            ),
            (search_query, query.limit, query.limit * (query.page - 1)),
        )
        .map(Into::<api::User>::into)
        .collect::<Vec<_>>();
    Response::new().json(api::UserIndexResponse {
        pagination: api::Pagination {
            total,
            page: query.page,
            limit: query.limit,
        },
        data: users,
    })
}

// MARK: Users create
#[derive(Validate)]
#[validate(context(Context))]
struct UserCreateBody {
    #[validate(ascii, length(min = 1, max = 32), custom(is_unique_username))]
    username: String,
    #[validate(email, length(max = 128), custom(is_unique_email))]
    email: String,
    #[validate(ascii, length(min = 6, max = 128))]
    password: String,
}

impl From<api::UserCreateBody> for UserCreateBody {
    fn from(body: api::UserCreateBody) -> Self {
        Self {
            username: body.username,
            email: body.email,
            password: body.password,
        }
    }
}

pub fn users_create(req: &Request, ctx: &Context) -> Response {
    // Authorization
    // -

    // Parse and validate body
    let body = match serde_urlencoded::from_bytes::<api::UserCreateBody>(
        req.body.as_deref().unwrap_or(&[]),
    ) {
        Ok(body) => Into::<UserCreateBody>::into(body),
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
        password: password_hash(&body.password),
        role: UserRole::Normal,
        ..Default::default()
    };
    ctx.database.insert_user(user.clone());

    Response::new().json(Into::<api::User>::into(user))
}

// MARK: Users show
pub fn users_show(req: &Request, ctx: &Context) -> Response {
    let user = match find_user(req, ctx) {
        Some(user) => user,
        None => return not_found(req, ctx),
    };

    // Authorization
    // -

    Response::new().json(Into::<api::User>::into(user))
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

pub fn users_update(req: &Request, ctx: &Context) -> Response {
    let mut user = match find_user(req, ctx) {
        Some(user) => user,
        None => return not_found(req, ctx),
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
    let body = match serde_urlencoded::from_bytes::<Body>(req.body.as_deref().unwrap_or(&[])) {
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
    user.username = body.username;
    user.email = body.email;
    user.firstname = body.firstname;
    user.lastname = body.lastname;
    user.birthdate = body
        .birthdate
        .and_then(|birthdate| NaiveDate::from_str(&birthdate).ok());
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

    Response::new().json(Into::<api::User>::into(user))
}

// MARK: Users change password

#[derive(Validate)]
#[validate(context(Context))]
struct UserUpdatePasswordBody {
    #[validate(ascii, custom(is_auth_user_current_password))]
    current_password: String,
    #[validate(ascii, length(min = 6, max = 128))]
    password: String,
}

impl From<api::UserUpdatePasswordBody> for UserUpdatePasswordBody {
    fn from(body: api::UserUpdatePasswordBody) -> Self {
        Self {
            current_password: body.current_password,
            password: body.password,
        }
    }
}

pub fn users_change_password(req: &Request, ctx: &Context) -> Response {
    let mut user = match find_user(req, ctx) {
        Some(user) => user,
        None => return not_found(req, ctx),
    };

    // Authorization
    let auth_user = ctx.auth_user.as_ref().expect("Not authed");
    if !(user.id == auth_user.id || auth_user.role == UserRole::Admin) {
        return Response::new()
            .status(Status::Unauthorized)
            .body("401 Unauthorized");
    }

    // Parse and validate body
    let body = match serde_urlencoded::from_bytes::<api::UserUpdatePasswordBody>(
        req.body.as_deref().unwrap_or(&[]),
    ) {
        Ok(body) => Into::<UserUpdatePasswordBody>::into(body),
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
    user.password = password_hash(&body.password);
    user.updated_at = Utc::now();
    ctx.database.execute(
        "UPDATE users SET password = ?, updated_at = ? WHERE id = ?",
        (user.password.clone(), user.updated_at, user.id),
    );

    Response::new().json(Into::<api::User>::into(user))
}

// MARK: Users sessions
pub fn users_sessions(req: &Request, ctx: &Context) -> Response {
    let user = match find_user(req, ctx) {
        Some(user) => user,
        None => return not_found(req, ctx),
    };

    // Authorization
    let auth_user = ctx.auth_user.as_ref().expect("Not authed");
    if !(user.id == auth_user.id || auth_user.role == UserRole::Admin) {
        return Response::new()
            .status(Status::Unauthorized)
            .body("401 Unauthorized");
    }

    // Parse index query
    let query = match req.url.query() {
        Some(query) => match serde_urlencoded::from_str::<IndexQuery>(query) {
            Ok(query) => query,
            Err(_) => return Response::with_status(Status::BadRequest),
        },
        None => IndexQuery::default(),
    };
    if let Err(report) = query.validate() {
        return Response::with_status(Status::BadRequest).json(report);
    }

    let total = ctx
        .database
        .query::<i64>(
            "SELECT COUNT(id) FROM sessions WHERE user_id = ? AND expires_at > ?",
            (user.id, Utc::now()),
        )
        .next()
        .expect("Can't count sessions");
    let user_sessions = ctx
            .database
            .query::<Session>(
                formatcp!(
                    "SELECT {} FROM sessions WHERE user_id = ? AND expires_at > ? ORDER BY expires_at DESC LIMIT ? OFFSET ?",
                    Session::columns()
                ),
                (user.id, Utc::now(), query.limit, query.limit * (query.page - 1)),
            )
            .map(Into::<api::Session>::into)
            .collect::<Vec<_>>();
    Response::new().json(api::SessionIndexResponse {
        pagination: api::Pagination {
            total,
            page: query.page,
            limit: query.limit,
        },
        data: user_sessions,
    })
}

// MARK: Users posts
pub fn users_posts(req: &Request, ctx: &Context) -> Response {
    let user = match find_user(req, ctx) {
        Some(user) => user,
        None => return not_found(req, ctx),
    };

    // Authorization
    // -

    // Parse request query
    let query = match req.url.query() {
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
    let search_query = format!("%{}%", query.query.replace("%", "\\%"));
    let total = ctx
        .database
        .query::<i64>(
            "SELECT COUNT(id) FROM posts WHERE user_id = ? AND text LIKE ?",
            (user.id, search_query.clone()),
        )
        .next()
        .expect("Can't count posts");
    let user_posts = ctx
        .database
        .query::<Post>(
            formatcp!(
                "SELECT {} FROM posts WHERE user_id = ? AND text LIKE ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
                Post::columns()
            ),
            (
                user.id,
               search_query,
               query. limit,
               query. limit * (query.page - 1),
            )
        )
        .map(|mut post| {
            post.fetch_relationships(ctx);
            post
        })
        .map(Into::<api::Post>::into)
        .collect::<Vec<_>>();
    Response::new().json(api::PostIndexResponse {
        pagination: api::Pagination {
            total,
            page: query.page,
            limit: query.limit,
        },
        data: user_posts,
    })
}

#[cfg(test)]
mod test {
    use small_http::Method;

    use super::*;
    use crate::controllers::auth::generate_random_token;
    use crate::router;
    use crate::test_utils::{create_user, create_user_session};

    // MARK: Test Users index
    #[test]
    fn test_users_index() {
        let ctx = Context::with_test_database();
        let router = router(ctx.clone());
        let (_, session) = create_user_session(&ctx, UserRole::Admin);

        for _ in 0..10 {
            create_user(&ctx, UserRole::Normal);
        }

        let req = Request::with_url("http://localhost/users")
            .header("Authorization", format!("Bearer {}", session.token));
        let res = router.handle(&req);
        assert_eq!(res.status, Status::Ok);
        let res = serde_json::from_slice::<api::UserIndexResponse>(&res.body).unwrap();
        assert_eq!(res.pagination.total, 10 + 1);
    }

    // MARK: Test Users create
    #[test]
    fn test_users_create() {
        let ctx = Context::with_test_database();
        let router = router(ctx.clone());
        let (_, session) = create_user_session(&ctx, UserRole::Admin);

        let req = Request::with_url("http://localhost/users")
            .method(Method::Post)
            .header("Authorization", format!("Bearer {}", session.token))
            .body(
                serde_urlencoded::to_string(api::UserCreateBody {
                    username: "newuser".to_string(),
                    email: "newuser@example.com".to_string(),
                    password: "password123".to_string(),
                })
                .unwrap(),
            );
        let res = router.handle(&req);
        assert_eq!(res.status, Status::Ok);
        let res = serde_json::from_slice::<api::User>(&res.body).unwrap();
        assert_eq!(res.username, "newuser");
        assert_eq!(res.email, "newuser@example.com");
    }

    // MARK: Test Users show
    #[test]
    fn test_users_show() {
        let ctx = Context::with_test_database();
        let router = router(ctx.clone());
        let (user, session) = create_user_session(&ctx, UserRole::Normal);

        let req = Request::with_url(format!("http://localhost/users/{}", user.id))
            .header("Authorization", format!("Bearer {}", session.token));
        let res = router.handle(&req);
        assert_eq!(res.status, Status::Ok);
        let res = serde_json::from_slice::<api::User>(&res.body).unwrap();
        assert_eq!(res.id, user.id);
    }

    // MARK: Test Users update
    #[test]
    fn test_users_update() {
        let ctx = Context::with_test_database();
        let router = router(ctx.clone());
        let (user, session) = create_user_session(&ctx, UserRole::Normal);

        let req = Request::with_url(format!("http://localhost/users/{}", user.id))
            .method(Method::Put)
            .header("Authorization", format!("Bearer {}", session.token))
            .body(
                serde_urlencoded::to_string(api::UserUpdateBody {
                    username: "updateduser".to_string(),
                    email: "updateduser@example.com".to_string(),
                    firstname: Some("Updated".to_string()),
                    lastname: Some("User".to_string()),
                    birthdate: Some("2000-01-01".to_string()),
                    bio: Some("Updated bio".to_string()),
                    location: Some("Updated location".to_string()),
                    website: Some("http://updatedwebsite.com".to_string()),
                })
                .unwrap(),
            );
        let res = router.handle(&req);
        assert_eq!(res.status, Status::Ok);
        let res = serde_json::from_slice::<api::User>(&res.body).unwrap();
        assert_eq!(res.username, "updateduser");
        assert_eq!(res.email, "updateduser@example.com");
    }

    // MARK: Test Users change password
    #[test]
    fn test_users_change_password() {
        let ctx = Context::with_test_database();
        let router = router(ctx.clone());

        let user = User {
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password: password_hash("password"),
            ..Default::default()
        };
        ctx.database.insert_user(user.clone());
        let session = Session {
            user_id: user.id,
            token: generate_random_token(),
            ..Default::default()
        };
        ctx.database.insert_session(session.clone());

        let req = Request::with_url(format!(
            "http://localhost/users/{}/change_password",
            user.id
        ))
        .method(Method::Put)
        .header("Authorization", format!("Bearer {}", session.token))
        .body(
            serde_urlencoded::to_string(api::UserUpdatePasswordBody {
                current_password: "password".to_string(),
                password: "password123".to_string(),
            })
            .unwrap(),
        );
        let res = router.handle(&req);
        assert_eq!(res.status, Status::Ok);
    }

    // MARK: Test Users sessions
    #[test]
    fn test_users_sessions() {
        let ctx = Context::with_test_database();
        let router = router(ctx.clone());
        let (user, session) = create_user_session(&ctx, UserRole::Normal);

        let req = Request::with_url(format!("http://localhost/users/{}/sessions", user.id))
            .header("Authorization", format!("Bearer {}", session.token));
        let res = router.handle(&req);
        assert_eq!(res.status, Status::Ok);
        let res = serde_json::from_slice::<api::SessionIndexResponse>(&res.body).unwrap();
        assert!(!res.data.is_empty());
    }

    // MARK: Test Users posts
    #[test]
    fn test_users_posts() {
        let ctx = Context::with_test_database();
        let router = router(ctx.clone());
        let (user, session) = create_user_session(&ctx, UserRole::Normal);

        ctx.database.insert_post(Post {
            user_id: user.id,
            text: "This is a test post".to_string(),
            ..Default::default()
        });

        let req = Request::with_url(format!("http://localhost/users/{}/posts", user.id))
            .header("Authorization", format!("Bearer {}", session.token));
        let res = router.handle(&req);
        assert_eq!(res.status, Status::Ok);
        let res = serde_json::from_slice::<api::PostIndexResponse>(&res.body).unwrap();
        assert!(!res.data.is_empty());
    }
}
