/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { User } from './user.ts';

export interface Post {
    id: string;
    type: PostType;
    text: string;
    replies_count: number;
    reposts_count: number;
    likes_count: number;
    dislikes_count: number;
    views_count: number;
    created_at: string;
    updated_at: string;
    text_html?: string;
    parent_post?: Post;
    user?: User;
    replies?: Post[];
    auth_user_liked?: boolean;
    auth_user_disliked?: boolean;
}

export enum PostType {
    NORMAL = 'normal',
    REPLY = 'reply',
    REPOST = 'repost',
}
