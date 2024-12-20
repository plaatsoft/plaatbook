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
    replies: number;
    reposts: number;
    likes: number;
    dislikes: number;
    views: number;
    created_at: string;
    updated_at: string;
    parent_post?: Post;
    user?: User;
    auth_user_reposted?: boolean;
    auth_user_liked?: boolean;
    auth_user_disliked?: boolean;
}

export enum PostType {
    NORMAL = 'normal',
    REPLY = 'reply',
    REPOST = 'repost',
}
