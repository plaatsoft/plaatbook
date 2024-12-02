/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

use super::not_found;
use crate::models::user::User;

pub async fn users_index(State(database): State<Pool<Sqlite>>) -> impl IntoResponse {
    Json(
        sqlx::query_as!(
            User,
            "SELECT id as \"id: uuid::Uuid\", username, email, password, created_at, updated_at FROM users"
        )
        .fetch_all(&database)
        .await
        .unwrap(),
    )
}

pub async fn users_store(State(database): State<Pool<Sqlite>>) -> impl IntoResponse {
    // Create a new user
    let user = User {
        id: Uuid::now_v7(),
        username: "plaatsoft".to_string(),
        email: "info@plaatsoft.nl".to_string(),
        password: "password".to_string(),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };
    sqlx::query!(
            "INSERT INTO users (id, username, email, password, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)",
            user.id,
            user.username,
            user.email,
            user.password,
            user.created_at,
            user.updated_at,
        )
        .fetch_optional(&database)
        .await
        .unwrap();

    (StatusCode::CREATED, Json(user)).into_response()
}

pub async fn users_show(
    State(database): State<Pool<Sqlite>>,
    Path(user_id): Path<Uuid>,
) -> impl IntoResponse {
    if let Ok(user) =
        sqlx::query_as!(
            User,
            "SELECT id as \"id: uuid::Uuid\", username, email, password, created_at, updated_at FROM users WHERE id = ?"
        ,user_id)
        .fetch_one(&database)
        .await
    {
       (StatusCode::OK, Json(user)).into_response()
    } else {
        not_found().await.into_response()
    }
}
