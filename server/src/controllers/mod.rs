/*
 * Copyright (c) 2024-2025 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use http::{Request, Response, Status};

use crate::{api, Context};

pub mod auth;
pub mod posts;
pub mod sessions;
pub mod users;

// MARK: Home
pub fn home(_: &Request, _: &Context) -> Response {
    Response::new().json(api::HomeResponse {
        name: "PlaatBook".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

// MARK: Not Found
pub fn not_found(_: &Request, _: &Context) -> Response {
    Response::new()
        .status(Status::NotFound)
        .body("404 Not Found")
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::router;

    // MARK: Test Home
    #[test]
    fn test_home() {
        let ctx = Context::with_test_database();
        let router = router(ctx.clone());

        let res = router.handle(&Request::with_url("http://localhost/"));
        assert_eq!(res.status, Status::Ok);
        let json = serde_json::from_slice::<api::HomeResponse>(&res.body).unwrap();
        assert_eq!(json.name, "PlaatBook");
        assert_eq!(json.version, env!("CARGO_PKG_VERSION"));
    }

    // MARK: Test Not Found
    #[test]
    fn test_not_found() {
        let ctx = Context::with_test_database();
        let router = router(ctx.clone());

        let res = router.handle(&Request::with_url("http://localhost/nonexistent"));
        assert_eq!(res.status, Status::NotFound);
    }
}
