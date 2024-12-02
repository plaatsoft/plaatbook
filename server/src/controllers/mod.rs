/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use axum::http::StatusCode;
use axum::response::IntoResponse;

pub mod users;

pub async fn home() -> impl IntoResponse {
    concat!("PlaatBook v", env!("CARGO_PKG_VERSION"))
}

pub async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404 Not Found")
}
