/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use std::net::Ipv4Addr;

use anyhow::Result;
use axum::routing::get;
use axum::Router;
use tower_http::cors::CorsLayer;

use crate::controllers::{home, not_found};

mod controllers;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/", get(home))
        .fallback(not_found)
        .layer(CorsLayer::new().allow_origin(tower_http::cors::Any));

    let listener = tokio::net::TcpListener::bind((Ipv4Addr::UNSPECIFIED, 8080)).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
