/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { User } from './user.ts';

export interface Post {
    id: string;
    text: string;
    likes: number;
    dislikes: number;
    created_at: string;
    updated_at: string;
    user?: User;
    auth_user_liked?: boolean;
    auth_user_disliked?: boolean;
}
