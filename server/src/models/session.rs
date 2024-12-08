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
use crate::consts::SESSION_EXPIRE_DURATION;

#[derive(Clone, Serialize, FromRow)]
pub struct Session {
    pub id: Uuid,
    #[serde(skip)]
    pub user_id: Uuid,
    #[serde(skip)]
    pub token: String,
    pub ip_address: String,
    pub ip_latitude: Option<f64>,
    pub ip_longitude: Option<f64>,
    pub ip_country: Option<String>,
    pub ip_city: Option<String>,
    pub client_name: Option<String>,
    pub client_version: Option<String>,
    pub client_os: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[sqlite(skip)]
    pub user: Option<User>,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            id: Uuid::now_v7(),
            user_id: Uuid::now_v7(),
            token: String::new(),
            ip_address: String::new(),
            ip_latitude: None,
            ip_longitude: None,
            ip_country: None,
            ip_city: None,
            client_name: None,
            client_version: None,
            client_os: None,
            expires_at: Utc::now() + SESSION_EXPIRE_DURATION,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            user: None,
        }
    }
}
