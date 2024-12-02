/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use std::net::Ipv4Addr;

use anyhow::Result;
use axum::routing::{get, post};
use axum::Router;
use controllers::users::{users_show, users_store};
use sqlx::SqlitePool;
use tower_http::cors::CorsLayer;

use crate::controllers::users::users_index;
use crate::controllers::{home, not_found};

mod controllers;
mod models;

const HTTP_PORT: u16 = 8080;
const DATABASE_URL: &str = "sqlite://database.db";

#[tokio::main]
async fn main() -> Result<()> {
    let database = SqlitePool::connect(DATABASE_URL).await?;

    let app = Router::new()
        .route("/", get(home))
        // Users
        .route("/users", get(users_index))
        .route("/users", post(users_store))
        .route("/users/:user_id", get(users_show))
        .fallback(not_found)
        .layer(CorsLayer::new().allow_origin(tower_http::cors::Any))
        .with_state(database);

    let listener = tokio::net::TcpListener::bind((Ipv4Addr::UNSPECIFIED, HTTP_PORT)).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
