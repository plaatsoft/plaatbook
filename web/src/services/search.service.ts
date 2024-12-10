/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { Post } from '../models/post.ts';
import { User } from '../models/user.ts';
import { $authUser } from './auth.service.ts';

export class SearchService {
    static instance?: SearchService;

    static getInstance(): SearchService {
        if (SearchService.instance === undefined) {
            SearchService.instance = new SearchService();
        }
        return SearchService.instance;
    }

    async search(query: string): Promise<{ posts: Post[]; users: User[] } | null> {
        const headers = new Headers();
        if ($authUser.value !== null) {
            headers.append('Authorization', `Bearer ${localStorage.getItem('token')}`);
        }
        const res = await fetch(`${import.meta.env.VITE_API_URL}/search?q=${encodeURIComponent(query)}`, { headers });
        if (res.status !== 200) {
            return null;
        }
        return (await res.json()) as { posts: Post[]; users: User[] };
    }
}