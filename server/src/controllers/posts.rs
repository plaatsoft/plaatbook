/*
 * Copyright (c) 2024-2025 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use chrono::Utc;
use const_format::formatcp;
use http::{Request, Response, Status};
use uuid::Uuid;
use validate::Validate;

use crate::controllers::not_found;
use crate::database::Extension;
use crate::models::{
    IndexQuery, Post, PostInteraction, PostInteractionType, PostType, User, UserRole,
};
use crate::{api, Context};

// MARK: Helpers
fn find_post(req: &Request, ctx: &Context) -> Option<Post> {
    let post_id = match req
        .params
        .get("post_id")
        .expect("Should exists")
        .parse::<Uuid>()
    {
        Ok(id) => id,
        Err(_) => return None,
    };

    ctx.database
        .query::<Post>(
            formatcp!("SELECT {} FROM posts WHERE id = ? LIMIT 1", Post::columns()),
            post_id,
        )
        .next()
}

fn remove_post_like(database: &bsqlite::Connection, post_id: Uuid, auth_user: &User) {
    // Remove post like interaction
    database.execute(
        "DELETE FROM post_interactions WHERE post_id = ? AND user_id = ? AND type = ?",
        (post_id, auth_user.id, PostInteractionType::Like),
    );
    if database.affected_rows() > 0 {
        database.execute("UPDATE posts SET likes = likes - 1 WHERE id = ?", post_id);
    }
}

fn remove_post_dislike(database: &bsqlite::Connection, post_id: Uuid, auth_user: &User) {
    // Remove post dislike interaction
    database.execute(
        "DELETE FROM post_interactions WHERE post_id = ? AND user_id = ? AND type = ?",
        (post_id, auth_user.id, PostInteractionType::Dislike),
    );
    if database.affected_rows() > 0 {
        database.execute(
            "UPDATE posts SET dislikes = dislikes - 1 WHERE id = ?",
            post_id,
        );
    }
}

// MARK: Posts index
pub fn posts_index(req: &Request, ctx: &Context) -> Response {
    // Authorization
    // -

    // Parse index query
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

    // Get posts
    let search_query = format!("%{}%", query.query.replace("%", "\\%"));
    let total = ctx
        .database
        .query::<i64>(
            "SELECT COUNT(id) FROM posts WHERE text LIKE ?",
            search_query.clone(),
        )
        .next()
        .expect("Can't count posts");
    let posts = ctx
        .database
        .query::<Post>(
            formatcp!(
                "SELECT {} FROM posts WHERE text LIKE ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
                Post::columns()
            ),
            (search_query, query.limit, query.limit * (query.page - 1)),
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
        data: posts,
    })
}

// MARK: Posts create
#[derive(Validate)]
struct PostCreateUpdateBody {
    #[validate(length(min = 1, max = 512))]
    text: String,
}

impl From<api::PostCreateUpdateBody> for PostCreateUpdateBody {
    fn from(body: api::PostCreateUpdateBody) -> Self {
        Self { text: body.text }
    }
}

pub fn posts_create(req: &Request, ctx: &Context) -> Response {
    // Authorization
    let auth_user = match ctx.auth_user.as_ref() {
        Some(user) => user,
        None => {
            return Response::new()
                .status(Status::Unauthorized)
                .body("401 Unauthorized")
        }
    };

    // Parse and validate body
    let body = match serde_urlencoded::from_bytes::<api::PostCreateUpdateBody>(
        req.body.as_deref().unwrap_or(&[]),
    ) {
        Ok(body) => Into::<PostCreateUpdateBody>::into(body),
        Err(_) => {
            return Response::new()
                .status(Status::BadRequest)
                .body("400 Bad Request");
        }
    };
    if let Err(errors) = body.validate() {
        return Response::new().status(Status::BadRequest).json(errors);
    }

    // Create new post
    let mut post = Post {
        user_id: auth_user.id,
        text: body.text,
        ..Default::default()
    };
    ctx.database.insert_post(post.clone());

    // Return new post
    post.fetch_relationships(ctx);
    Response::new().json(Into::<api::Post>::into(post))
}

// MARK: Posts show
pub fn posts_show(req: &Request, ctx: &Context) -> Response {
    let mut post = match find_post(req, ctx) {
        Some(post) => post,
        None => return not_found(req, ctx),
    };

    // Authorization
    // -

    // Fetch post replies
    let replies = ctx
        .database
        .query::<Post>(
            formatcp!(
                "SELECT {} FROM posts WHERE parent_post_id = ? AND type = ? ORDER BY created_at DESC",
                Post::columns()
            ),
            (
                post.id,
                PostType::Reply,
            )
        )
        .map(|mut reply| {
            reply.fetch_relationships(ctx);
            reply
        })
        .collect::<Vec<_>>();
    post.replies = Some(replies);

    post.fetch_relationships(ctx);
    Response::new().json(Into::<api::Post>::into(post))
}

// MARK: Posts update
pub fn posts_update(req: &Request, ctx: &Context) -> Response {
    let mut post = match find_post(req, ctx) {
        Some(post) => post,
        None => return not_found(req, ctx),
    };

    // Authorization
    let auth_user = ctx.auth_user.as_ref().expect("Not authed");
    if !(post.user_id == auth_user.id || auth_user.role == UserRole::Admin) {
        return Response::new()
            .status(Status::Unauthorized)
            .body("401 Unauthorized");
    }

    // Parse and validate body
    let body = match serde_urlencoded::from_bytes::<api::PostCreateUpdateBody>(
        req.body.as_deref().unwrap_or(&[]),
    ) {
        Ok(body) => Into::<PostCreateUpdateBody>::into(body),
        Err(_) => {
            return Response::new()
                .status(Status::BadRequest)
                .body("400 Bad Request");
        }
    };
    if let Err(errors) = body.validate() {
        return Response::new().status(Status::BadRequest).json(errors);
    }

    // Update post
    post.text = body.text;
    post.updated_at = Utc::now();
    ctx.database.execute(
        "UPDATE posts SET text = ?, updated_at = ? WHERE id = ? OR parent_post_id = ?",
        (post.text.clone(), post.updated_at, post.id, post.id),
    );

    // Return updated post
    post.fetch_relationships(ctx);
    Response::new().json(Into::<api::Post>::into(post))
}

// MARK: Posts delete
pub fn posts_delete(req: &Request, ctx: &Context) -> Response {
    let post = match find_post(req, ctx) {
        Some(post) => post,
        None => return not_found(req, ctx),
    };

    // Authorization
    let auth_post = ctx.auth_user.as_ref().expect("Not authed");
    if !(post.user_id == auth_post.id || auth_post.role == UserRole::Admin) {
        return Response::new()
            .status(Status::Unauthorized)
            .body("401 Unauthorized");
    }

    // Update parent post counters
    if post.r#type == PostType::Reply {
        ctx.database.execute(
            "UPDATE posts SET replies = replies - 1 WHERE id = ?",
            post.parent_post_id,
        );
    }
    if post.r#type == PostType::Repost {
        ctx.database.execute(
            "UPDATE posts SET reposts = reposts - 1 WHERE id = ?",
            post.parent_post_id,
        );
    }

    // Delete post
    ctx.database
        .execute("DELETE FROM posts WHERE id = ?", post.id);

    Response::new()
}

// MARK: Posts replies
pub fn posts_replies(req: &Request, ctx: &Context) -> Response {
    let post = match find_post(req, ctx) {
        Some(post) => post,
        None => return not_found(req, ctx),
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

    // Get post replies
    let search_query = format!("%{}%", query.query.replace("%", "\\%"));
    let total = ctx
        .database
        .query::<i64>(
            "SELECT COUNT(id) FROM posts WHERE parent_post_id = ? AND text LIKE ?",
            (post.id, search_query.clone()),
        )
        .next()
        .expect("Can't count posts");
    let posts = ctx
        .database
        .query::<Post>(
            formatcp!(
                "SELECT {} FROM posts WHERE parent_post_id = ? AND text LIKE ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
                Post::columns()
            ),
            (
                post.id,
                search_query,
                query.limit,
                query. limit * (query.page - 1),
            ),
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
        data: posts,
    })
}

// MARK: Post create reply
pub fn posts_create_reply(req: &Request, ctx: &Context) -> Response {
    let post = match find_post(req, ctx) {
        Some(post) => post,
        None => return not_found(req, ctx),
    };

    // Authorization
    let auth_user = match ctx.auth_user.as_ref() {
        Some(user) => user,
        None => {
            return Response::new()
                .status(Status::Unauthorized)
                .body("401 Unauthorized")
        }
    };

    // Parse and validate body
    let body = match serde_urlencoded::from_bytes::<api::PostCreateUpdateBody>(
        req.body.as_deref().unwrap_or(&[]),
    ) {
        Ok(body) => Into::<PostCreateUpdateBody>::into(body),
        Err(_) => {
            return Response::new()
                .status(Status::BadRequest)
                .body("400 Bad Request");
        }
    };
    if let Err(errors) = body.validate() {
        return Response::new().status(Status::BadRequest).json(errors);
    }

    // Create new reply post
    let mut reply = Post {
        r#type: PostType::Reply,
        parent_post_id: Some(post.id),
        user_id: auth_user.id,
        text: body.text,
        ..Default::default()
    };
    ctx.database.insert_post(reply.clone());

    // Update parent post replies counter
    ctx.database.execute(
        "UPDATE posts SET replies = replies + 1 WHERE id = ?",
        post.id,
    );

    // Return new reply
    reply.fetch_relationships(ctx);
    Response::new().json(Into::<api::Post>::into(reply))
}

// MARK: Posts repost
pub fn posts_repost(req: &Request, ctx: &Context) -> Response {
    let post = match find_post(req, ctx) {
        Some(post) => post,
        None => return not_found(req, ctx),
    };

    // Authorization
    let auth_user = match ctx.auth_user.as_ref() {
        Some(user) => user,
        None => {
            return Response::new()
                .status(Status::Unauthorized)
                .body("401 Unauthorized")
        }
    };

    // Create new repost
    let mut repost = Post {
        r#type: PostType::Repost,
        parent_post_id: Some(post.content_post_id()),
        user_id: auth_user.id,
        text: post.text.clone(),
        ..Default::default()
    };
    ctx.database.insert_post(repost.clone());

    // Update content post reposts counter
    ctx.database.execute(
        "UPDATE posts SET reposts = reposts + 1 WHERE id = ?",
        post.content_post_id(),
    );

    // Return new repost
    repost.fetch_relationships(ctx);
    Response::new().json(Into::<api::Post>::into(repost))
}

// MARK: Posts like
pub fn posts_like(req: &Request, ctx: &Context) -> Response {
    let post = match find_post(req, ctx) {
        Some(post) => post,
        None => return not_found(req, ctx),
    };

    // Authorization
    let auth_user = match ctx.auth_user.as_ref() {
        Some(user) => user,
        None => {
            return Response::new()
                .status(Status::Unauthorized)
                .body("401 Unauthorized")
        }
    };

    // Remove possible old post interaction
    remove_post_like(&ctx.database, post.content_post_id(), auth_user);
    remove_post_dislike(&ctx.database, post.content_post_id(), auth_user);

    // Create new post like interaction
    let post_interaction = PostInteraction {
        post_id: post.content_post_id(),
        user_id: auth_user.id,
        r#type: PostInteractionType::Like,
        ..Default::default()
    };
    ctx.database.execute(
        formatcp!(
            "INSERT INTO post_interactions ({}) VALUES ({})",
            PostInteraction::columns(),
            PostInteraction::values()
        ),
        post_interaction,
    );
    ctx.database.execute(
        "UPDATE posts SET likes = likes + 1 WHERE id = ?",
        post.content_post_id(),
    );

    Response::new()
}

// MARK: Posts like delete
pub fn posts_like_delete(req: &Request, ctx: &Context) -> Response {
    let post = match find_post(req, ctx) {
        Some(post) => post,
        None => return not_found(req, ctx),
    };

    // Authorization
    let auth_user = match ctx.auth_user.as_ref() {
        Some(user) => user,
        None => {
            return Response::new()
                .status(Status::Unauthorized)
                .body("401 Unauthorized")
        }
    };

    // Remove post like
    remove_post_like(&ctx.database, post.content_post_id(), auth_user);
    Response::new()
}

// MARK: Posts dislike
pub fn posts_dislike(req: &Request, ctx: &Context) -> Response {
    let post = match find_post(req, ctx) {
        Some(post) => post,
        None => return not_found(req, ctx),
    };

    // Authorization
    let auth_user = match ctx.auth_user.as_ref() {
        Some(user) => user,
        None => {
            return Response::new()
                .status(Status::Unauthorized)
                .body("401 Unauthorized")
        }
    };

    // Remove possible old post interaction
    remove_post_like(&ctx.database, post.content_post_id(), auth_user);
    remove_post_dislike(&ctx.database, post.content_post_id(), auth_user);

    // Create new post dislike interaction
    let post_interaction = PostInteraction {
        post_id: post.content_post_id(),
        user_id: auth_user.id,
        r#type: PostInteractionType::Dislike,
        ..Default::default()
    };
    ctx.database.execute(
        formatcp!(
            "INSERT INTO post_interactions ({}) VALUES ({})",
            PostInteraction::columns(),
            PostInteraction::values()
        ),
        post_interaction,
    );
    ctx.database.execute(
        "UPDATE posts SET dislikes = dislikes + 1 WHERE id = ?",
        post.content_post_id(),
    );

    Response::new()
}

// MARK: Posts dislike delete
pub fn posts_dislike_delete(req: &Request, ctx: &Context) -> Response {
    let post = match find_post(req, ctx) {
        Some(post) => post,
        None => return not_found(req, ctx),
    };

    // Authorization
    let auth_user = match ctx.auth_user.as_ref() {
        Some(user) => user,
        None => {
            return Response::new()
                .status(Status::Unauthorized)
                .body("401 Unauthorized")
        }
    };

    // Remove post dislike
    remove_post_dislike(&ctx.database, post.content_post_id(), auth_user);
    Response::new()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::router;
    use crate::test_utils::create_user_session;

    // MARK: Test Posts index
    #[test]
    fn test_posts_index() {
        let ctx = Context::with_test_database();
        let router = router(ctx.clone());
        let (user, session) = create_user_session(&ctx, UserRole::Admin);

        for _ in 0..10 {
            ctx.database.insert_post(Post {
                user_id: user.id,
                text: "Hello world".to_string(),
                ..Default::default()
            });
        }

        let req = Request::with_url("http://localhost/posts")
            .header("Authorization", format!("Bearer {}", session.token));
        let res = router.handle(&req);
        assert_eq!(res.status, Status::Ok);
        let res = serde_json::from_slice::<api::PostIndexResponse>(&res.body).unwrap();
        assert_eq!(res.pagination.total, 10);
    }

    // MARK: Test Posts create
    #[test]
    fn test_posts_create() {
        let ctx = Context::with_test_database();
        let router = router(ctx.clone());
        let (_, session) = create_user_session(&ctx, UserRole::Admin);

        let req = Request::with_url("http://localhost/posts")
            .method(http::Method::Post)
            .header("Authorization", format!("Bearer {}", session.token))
            .body("text=Hello%20world");
        let res = router.handle(&req);
        assert_eq!(res.status, Status::Ok);
        let res = serde_json::from_slice::<api::Post>(&res.body).unwrap();
        assert_eq!(&res.text, "Hello world");
    }

    // MARK: Test Posts show
    #[test]
    fn test_posts_show() {
        let ctx = Context::with_test_database();
        let router = router(ctx.clone());
        let (user, session) = create_user_session(&ctx, UserRole::Admin);

        let post = Post {
            user_id: user.id,
            text: "Hello world".to_string(),
            ..Default::default()
        };
        ctx.database.insert_post(post.clone());

        let req = Request::with_url(format!("http://localhost/posts/{}", post.id))
            .header("Authorization", format!("Bearer {}", session.token));
        let res = router.handle(&req);
        assert_eq!(res.status, Status::Ok);
        let res = serde_json::from_slice::<api::Post>(&res.body).unwrap();
        assert_eq!(res.id, post.id);
    }

    // MARK: Test Posts update
    #[test]
    fn test_posts_update() {
        let ctx = Context::with_test_database();
        let router = router(ctx.clone());
        let (user, session) = create_user_session(&ctx, UserRole::Admin);

        let post = Post {
            user_id: user.id,
            text: "Hello world".to_string(),
            ..Default::default()
        };
        ctx.database.insert_post(post.clone());

        let req = Request::with_url(format!("http://localhost/posts/{}", post.id))
            .method(http::Method::Put)
            .header("Authorization", format!("Bearer {}", session.token))
            .body("text=Updated%20text");
        let res = router.handle(&req);
        assert_eq!(res.status, Status::Ok);
        let res = serde_json::from_slice::<api::Post>(&res.body).unwrap();
        assert_eq!(&res.text, "Updated text");
    }

    // MARK: Test Posts delete
    #[test]
    fn test_posts_delete() {
        let ctx = Context::with_test_database();
        let router = router(ctx.clone());
        let (user, session) = create_user_session(&ctx, UserRole::Admin);

        let post = Post {
            user_id: user.id,
            text: "Hello world".to_string(),
            ..Default::default()
        };
        ctx.database.insert_post(post.clone());

        let req = Request::with_url(format!("http://localhost/posts/{}", post.id))
            .method(http::Method::Delete)
            .header("Authorization", format!("Bearer {}", session.token));
        let res = router.handle(&req);
        assert_eq!(res.status, Status::Ok);

        let req = Request::with_url(format!("http://localhost/posts/{}", post.id))
            .header("Authorization", format!("Bearer {}", session.token));
        let res = router.handle(&req);
        assert_eq!(res.status, Status::NotFound);
    }

    // MARK: Test Posts replies
    #[test]
    fn test_posts_replies() {
        let ctx = Context::with_test_database();
        let router = router(ctx.clone());
        let (user, session) = create_user_session(&ctx, UserRole::Admin);

        let post = Post {
            user_id: user.id,
            text: "Hello world".to_string(),
            ..Default::default()
        };
        ctx.database.insert_post(post.clone());

        for _ in 0..5 {
            ctx.database.insert_post(Post {
                user_id: user.id,
                text: "Reply".to_string(),
                parent_post_id: Some(post.id),
                r#type: PostType::Reply,
                ..Default::default()
            });
        }

        let req = Request::with_url(format!("http://localhost/posts/{}/replies", post.id))
            .header("Authorization", format!("Bearer {}", session.token));
        let res = router.handle(&req);
        assert_eq!(res.status, Status::Ok);
        let res = serde_json::from_slice::<api::PostIndexResponse>(&res.body).unwrap();
        assert_eq!(res.pagination.total, 5);
    }

    // MARK: Test Posts create reply
    #[test]
    fn test_posts_create_reply() {
        let ctx = Context::with_test_database();
        let router = router(ctx.clone());
        let (user, session) = create_user_session(&ctx, UserRole::Admin);

        let post = Post {
            user_id: user.id,
            text: "Hello world".to_string(),
            ..Default::default()
        };
        ctx.database.insert_post(post.clone());

        let req = Request::with_url(format!("http://localhost/posts/{}/reply", post.id))
            .method(http::Method::Post)
            .header("Authorization", format!("Bearer {}", session.token))
            .body("text=Reply%20text");
        let res = router.handle(&req);
        assert_eq!(res.status, Status::Ok);
        let res = serde_json::from_slice::<api::Post>(&res.body).unwrap();
        assert_eq!(&res.text, "Reply text");
    }

    // MARK: Test Posts repost
    #[test]
    fn test_posts_repost() {
        let ctx = Context::with_test_database();
        let router = router(ctx.clone());
        let (user, session) = create_user_session(&ctx, UserRole::Admin);

        let post = Post {
            user_id: user.id,
            text: "Hello world".to_string(),
            ..Default::default()
        };
        ctx.database.insert_post(post.clone());

        let req = Request::with_url(format!("http://localhost/posts/{}/repost", post.id))
            .method(http::Method::Post)
            .header("Authorization", format!("Bearer {}", session.token));
        let res = router.handle(&req);
        assert_eq!(res.status, Status::Ok);
        let res = serde_json::from_slice::<api::Post>(&res.body).unwrap();
        assert!(res.parent_post.is_some());
    }

    // MARK: Test Posts like
    #[test]
    fn test_posts_like() {
        let ctx = Context::with_test_database();
        let router = router(ctx.clone());
        let (user, session) = create_user_session(&ctx, UserRole::Admin);

        let post = Post {
            user_id: user.id,
            text: "Hello world".to_string(),
            ..Default::default()
        };
        ctx.database.insert_post(post.clone());

        let req = Request::with_url(format!("http://localhost/posts/{}/like", post.id))
            .method(http::Method::Put)
            .header("Authorization", format!("Bearer {}", session.token));
        let res = router.handle(&req);
        assert_eq!(res.status, Status::Ok);

        let post = ctx
            .database
            .query::<Post>("SELECT * FROM posts WHERE id = ?", post.id)
            .next()
            .unwrap();
        assert_eq!(post.likes_count, 1);
    }

    // MARK: Test Posts like delete
    #[test]
    fn test_posts_like_delete() {
        let ctx = Context::with_test_database();
        let router = router(ctx.clone());
        let (user, session) = create_user_session(&ctx, UserRole::Admin);

        let post = Post {
            user_id: user.id,
            text: "Hello world".to_string(),
            ..Default::default()
        };
        ctx.database.insert_post(post.clone());

        let req = Request::with_url(format!("http://localhost/posts/{}/like", post.id))
            .method(http::Method::Post)
            .header("Authorization", format!("Bearer {}", session.token));
        router.handle(&req);

        let req = Request::with_url(format!("http://localhost/posts/{}/like", post.id))
            .method(http::Method::Delete)
            .header("Authorization", format!("Bearer {}", session.token));
        let res = router.handle(&req);
        assert_eq!(res.status, Status::Ok);

        let post = ctx
            .database
            .query::<Post>("SELECT * FROM posts WHERE id = ?", post.id)
            .next()
            .unwrap();
        assert_eq!(post.likes_count, 0);
    }

    // MARK: Test Posts dislike
    #[test]
    fn test_posts_dislike() {
        let ctx = Context::with_test_database();
        let router = router(ctx.clone());
        let (user, session) = create_user_session(&ctx, UserRole::Admin);

        let post = Post {
            user_id: user.id,
            text: "Hello world".to_string(),
            ..Default::default()
        };
        ctx.database.insert_post(post.clone());

        let req = Request::with_url(format!("http://localhost/posts/{}/dislike", post.id))
            .method(http::Method::Put)
            .header("Authorization", format!("Bearer {}", session.token));
        let res = router.handle(&req);
        assert_eq!(res.status, Status::Ok);

        let post = ctx
            .database
            .query::<Post>("SELECT * FROM posts WHERE id = ?", post.id)
            .next()
            .unwrap();
        assert_eq!(post.dislikes_count, 1);
    }

    // MARK: Test Posts dislike delete
    #[test]
    fn test_posts_dislike_delete() {
        let ctx = Context::with_test_database();
        let router = router(ctx.clone());
        let (user, session) = create_user_session(&ctx, UserRole::Admin);

        let post = Post {
            user_id: user.id,
            text: "Hello world".to_string(),
            ..Default::default()
        };
        ctx.database.insert_post(post.clone());

        let req = Request::with_url(format!("http://localhost/posts/{}/dislike", post.id))
            .method(http::Method::Post)
            .header("Authorization", format!("Bearer {}", session.token));
        router.handle(&req);

        let req = Request::with_url(format!("http://localhost/posts/{}/dislike", post.id))
            .method(http::Method::Delete)
            .header("Authorization", format!("Bearer {}", session.token));
        let res = router.handle(&req);
        assert_eq!(res.status, Status::Ok);

        let post = ctx
            .database
            .query::<Post>("SELECT * FROM posts WHERE id = ?", post.id)
            .next()
            .unwrap();
        assert_eq!(post.dislikes_count, 0);
    }
}
