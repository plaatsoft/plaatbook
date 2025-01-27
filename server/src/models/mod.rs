/*
 * Copyright (c) 2024-2025 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;
use validate::Validate;

pub use self::post::{Post, PostType};
pub use self::post_interaction::{PostInteraction, PostInteractionType};
pub use self::session::Session;
pub use self::user::{User, UserRole};

pub mod post;
pub mod post_interaction;
pub mod session;
pub mod user;

// MARK: Index query
#[derive(Deserialize, Validate)]
#[serde(default)]
pub struct IndexQuery {
    #[serde(rename = "q")]
    pub query: String,
    #[validate(range(min = 1))]
    pub page: i64,
    #[validate(range(min = 1, max = 50))]
    pub limit: i64,
}

impl Default for IndexQuery {
    fn default() -> Self {
        Self {
            query: "".to_string(),
            page: 1,
            limit: 20,
        }
    }
}
