/*
 * Copyright (c) 2024-2025 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use std::time::Duration;

use bsqlite::FromRow;
use chrono::{DateTime, Utc};
use const_format::formatcp;
use uuid::Uuid;

use super::User;
use crate::{api, Context};

pub const SESSION_EXPIRE_DURATION: Duration = Duration::from_secs(365 * 24 * 60 * 60);

#[derive(Clone, FromRow)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
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
        let now = Utc::now();
        Self {
            id: Uuid::now_v7(),
            user_id: Uuid::nil(),
            token: String::new(),
            ip_address: String::new(),
            ip_latitude: None,
            ip_longitude: None,
            ip_country: None,
            ip_city: None,
            client_name: None,
            client_version: None,
            client_os: None,
            expires_at: now + SESSION_EXPIRE_DURATION,
            created_at: now,
            updated_at: now,
            user: None,
        }
    }
}

impl From<Session> for api::Session {
    fn from(session: Session) -> Self {
        Self {
            id: session.id,
            ip_address: session.ip_address,
            ip_latitude: session.ip_latitude,
            ip_longitude: session.ip_longitude,
            ip_country: session.ip_country,
            ip_city: session.ip_city,
            client_name: session.client_name,
            client_version: session.client_version,
            client_os: session.client_os,
            expires_at: session.expires_at,
            created_at: session.created_at,
            updated_at: session.updated_at,
            user: session.user.map(|user| user.into()),
        }
    }
}

// MARK: Relationships
impl Session {
    pub fn fetch_relationships(&mut self, ctx: &Context) {
        self.user = ctx
            .database
            .query::<User>(
                formatcp!("SELECT {} FROM users WHERE id = ? LIMIT 1", User::columns()),
                self.user_id,
            )
            .next();
    }
}
