/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlite::FromRow;
use uuid::Uuid;

use super::User;

#[derive(Clone, Serialize, FromRow)]
pub struct Post {
    pub id: Uuid,
    #[serde(skip)]
    pub user_id: Uuid,
    pub text: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[sqlite(skip)]
    pub user: Option<User>,
}

impl Default for Post {
    fn default() -> Self {
        Self {
            id: Uuid::now_v7(),
            user_id: Uuid::now_v7(),
            text: String::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            user: None,
        }
    }
}
