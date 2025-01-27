/*
 * Copyright (c) 2024-2025 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

use uuid::Uuid;

use crate::controllers::auth::generate_random_token;
use crate::database::Extension;
use crate::models::{Session, User, UserRole};
use crate::Context;

pub fn create_user(ctx: &Context, role: UserRole) -> User {
    let user = User {
        username: format!("test-{}", Uuid::now_v7()),
        email: format!("email-{}@example.com", Uuid::now_v7()),
        role,
        ..Default::default()
    };
    ctx.database.insert_user(user.clone());
    user
}

pub fn create_user_session(ctx: &Context, role: UserRole) -> (User, Session) {
    let user = create_user(ctx, role);
    let session = Session {
        user_id: user.id,
        token: generate_random_token(),
        ..Default::default()
    };
    ctx.database.insert_session(session.clone());
    (user, session)
}
