/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { signal } from '@preact/signals';
import { Errors } from '../models/errors.ts';
import { Post } from '../models/post.ts';
import { $authUser } from './auth.service.ts';

export const refreshPosts$ = signal<number>(0);

export class PostsService {
    static instance?: PostsService;

    static getInstance(): PostsService {
        if (PostsService.instance === undefined) {
            PostsService.instance = new PostsService();
        }
        return PostsService.instance;
    }

    async getAll(): Promise<Post[]> {
        const headers = new Headers();
        if ($authUser.value !== null) {
            headers.append('Authorization', `Bearer ${localStorage.getItem('token')}`);
        }
        const res = await fetch(`${import.meta.env.VITE_API_URL}/posts`, { headers });
        return (await res.json()) as Post[];
    }

    async create(text: string): Promise<Errors | null> {
        // Try to create a post with text
        const res = await fetch(`${import.meta.env.VITE_API_URL}/posts`, {
            method: 'POST',
            headers: {
                Authorization: `Bearer ${localStorage.getItem('token')}`,
            },
            body: new URLSearchParams({
                text,
            }),
        });
        if (res.status != 200) {
            return (await res.json()) as Errors;
        }
        return null;
    }

    async update(id: string, text: string): Promise<Errors | null> {
        // Try to update a post with text
        const res = await fetch(`${import.meta.env.VITE_API_URL}/posts/${id}`, {
            method: 'PUT',
            headers: {
                Authorization: `Bearer ${localStorage.getItem('token')}`,
            },
            body: new URLSearchParams({
                text,
            }),
        });
        if (res.status != 200) {
            return (await res.json()) as Errors;
        }
        return null;
    }

    async delete(id: string): Promise<void> {
        // Try to delete a post
        await fetch(`${import.meta.env.VITE_API_URL}/posts/${id}`, {
            method: 'DELETE',
            headers: {
                Authorization: `Bearer ${localStorage.getItem('token')}`,
            },
        });
    }
}
