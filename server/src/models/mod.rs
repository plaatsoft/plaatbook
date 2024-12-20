/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;
use validate::Validate;

pub use self::post::{Post, PostType};
pub use self::post_interaction::{PostInteraction, PostInteractionType};
pub use self::session::Session;
pub use self::user::{User, UserRole};
use crate::consts::LIMIT_MAX;

pub mod post;
pub mod post_interaction;
pub mod session;
pub mod user;

// MARK: Index query
#[derive(Default, Deserialize, Validate)]
pub struct IndexQuery {
    #[serde(rename = "q")]
    pub query: Option<String>,
    #[validate(range(min = 1))]
    pub page: Option<i64>,
    #[validate(range(min = 1, max = LIMIT_MAX))]
    pub limit: Option<i64>,
}
