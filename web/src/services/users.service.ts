/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { Errors } from '../models/errors.ts';
import { Post } from '../models/post.ts';
import { User } from '../models/user.ts';
import { $authToken, $authUser } from './auth.service.ts';

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
            headers.append('Authorization', `Bearer ${$authToken.value}`);
        }
        const res = await fetch(`${import.meta.env.VITE_API_URL}/users/${user_id}`, { headers });
        if (res.status !== 200) {
            return null;
        }
        return (await res.json()) as User;
    }

    async update(
        id: string,
        params: {
            firstname: string;
            lastname: string;
            username: string;
            email: string;
            birthdate: string;
            bio: string;
            location: string;
            website: string;
        },
    ): Promise<[boolean, User | Errors]> {
        const res = await fetch(`${import.meta.env.VITE_API_URL}/users/${id}`, {
            method: 'PUT',
            headers: {
                Authorization: `Bearer ${$authToken.value}`,
            },
            body: new URLSearchParams(params),
        });
        if (res.status == 200) {
            return [true, (await res.json()) as User];
        } else {
            return [false, (await res.json()) as Errors];
        }
    }

    async getPosts(user_id: string, page: number): Promise<Post[] | null> {
        const headers = new Headers();
        if ($authUser.value !== null) {
            headers.append('Authorization', `Bearer ${$authToken.value}`);
        }
        const res = await fetch(`${import.meta.env.VITE_API_URL}/users/${user_id}/posts?page=${page}`, {
            headers,
        });
        if (res.status !== 200) {
            return [];
        }
        return (await res.json()) as Post[];
    }
}
