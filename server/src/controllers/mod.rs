/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use axum::http::StatusCode;

pub async fn home() -> &'static str {
    concat!("PlaatBook v", env!("CARGO_PKG_VERSION"))
}

pub async fn not_found() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "404 Not Found")
}
