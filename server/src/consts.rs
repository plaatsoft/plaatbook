/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use std::time::Duration;

pub const DATABASE_PATH: &str = "database.db";
pub const HTTP_PORT: u16 = 8080;

pub const SESSION_EXPIRE_DURATION: Duration = Duration::from_secs(365 * 24 * 60 * 60);
