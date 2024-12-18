/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use http::{Method, Request, Response};

pub use crate::layers::auth::{auth_optional_layer, auth_required_layer};
use crate::Context;

mod auth;

// MARK: Log
pub fn log_layer(req: &Request, _: &mut Context) -> Option<Response> {
    println!("{} {}", req.method, req.url.path);
    None
}

// MARK: CORS
pub fn cors_pre_layer(req: &Request, ctx: &mut Context) -> Option<Response> {
    if req.method == Method::Options {
        Some(cors_post_layer(req, ctx, Response::new()))
    } else {
        None
    }
}

pub fn cors_post_layer(_: &Request, _: &mut Context, res: Response) -> Response {
    res.header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE")
        .header("Access-Control-Allow-Headers", "Authorization")
        .header("Access-Control-Max-Age", "86400")
}
