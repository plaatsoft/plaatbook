/*
 * Copyright (c) 2024-2025 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use http::{Method, Request, Response};

pub use crate::layers::auth::{auth_optional_pre_layer, auth_required_pre_layer};
use crate::Context;

mod auth;

// MARK: Log
pub fn log_pre_layer(req: &Request, _: &mut Context) -> Option<Response> {
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

// MARK: Tests
#[cfg(test)]
mod test {
    use super::*;
    use crate::router;

    #[test]
    fn test_cors() {
        let ctx = Context::with_test_database();
        let router = router(ctx.clone());

        let res = router.handle(&Request::with_url("http://localhost/"));
        assert_eq!(
            res.headers.get("Access-Control-Allow-Origin"),
            Some(&"*".to_string())
        );
    }

    #[test]
    fn test_cors_preflight() {
        let ctx = Context::with_test_database();
        let router = router(ctx.clone());

        let req = Request::with_url("http://localhost/").method(Method::Options);
        let res = router.handle(&req);
        assert_eq!(
            res.headers.get("Access-Control-Allow-Origin"),
            Some(&"*".to_string())
        );
        assert_eq!(
            res.headers.get("Access-Control-Allow-Methods"),
            Some(&"GET, POST, PUT, DELETE".to_string())
        );
        assert_eq!(
            res.headers.get("Access-Control-Allow-Headers"),
            Some(&"Authorization".to_string())
        );
        assert_eq!(
            res.headers.get("Access-Control-Max-Age"),
            Some(&"86400".to_string())
        );
    }
}
