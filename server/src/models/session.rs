/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlite::FromRow;
use uuid::Uuid;

#[derive(Clone, Serialize, FromRow)]
pub struct Session {
    pub id: Uuid,
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
}
