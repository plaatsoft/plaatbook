/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use http::{Request, Response, Status};
use router::Path;

use crate::{api, Context};

pub mod auth;
pub mod posts;
pub mod search;
pub mod sessions;
pub mod users;

pub fn home(_: &Request, _: &Context, _: &Path) -> Response {
    Response::new().json(api::ApiInfo {
        name: "PlaatBook".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

pub fn not_found(_: &Request, _: &Context, _: &Path) -> Response {
    Response::new()
        .status(Status::NotFound)
        .body("404 Not Found")
}
