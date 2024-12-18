/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { User } from './user.ts';

export interface Post {
    id: string;
    text: string;
    created_at: string;
    updated_at: string;
    user?: User;
    likes_count: number;
    dislikes_count: number;
    auth_user_liked?: boolean;
    auth_user_disliked?: boolean;
}
