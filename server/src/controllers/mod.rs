/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use anyhow::Result;
use http::{Request, Response, Status};
use router::Path;

use crate::Context;

pub mod auth;
pub mod posts;
pub mod sessions;
pub mod users;

pub fn home(_: &Request, _: &Context, _: &Path) -> Result<Response> {
    Ok(Response::new().body(concat!("PlaatBook v", env!("CARGO_PKG_VERSION"))))
}

pub fn not_found(_: &Request, _: &Context, _: &Path) -> Result<Response> {
    Ok(Response::new()
        .status(Status::NotFound)
        .body("404 Not Found"))
}
