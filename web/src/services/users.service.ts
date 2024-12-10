/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { Post } from '../models/post.ts';
import { User } from '../models/user.ts';
import { $authUser } from './auth.service.ts';

export class UsersService {
    static instance?: UsersService;

    static getInstance(): UsersService {
        if (UsersService.instance === undefined) {
            UsersService.instance = new UsersService();
        }
        return UsersService.instance;
    }

    async get(user_id: string): Promise<User | null> {
        const headers = new Headers();
        if ($authUser.value !== null) {
            headers.append('Authorization', `Bearer ${localStorage.getItem('token')}`);
        }
        const res = await fetch(`${import.meta.env.VITE_API_URL}/users/${user_id}`, { headers });
        if (res.status !== 200) {
            return null;
        }
        return (await res.json()) as User;
    }

    async getPosts(user_id: string): Promise<Post[] | null> {
        const headers = new Headers();
        if ($authUser.value !== null) {
            headers.append('Authorization', `Bearer ${localStorage.getItem('token')}`);
        }
        const res = await fetch(`${import.meta.env.VITE_API_URL}/users/${user_id}/posts`, { headers });
        if (res.status !== 200) {
            return [];
        }
        return (await res.json()) as Post[];
    }
}
