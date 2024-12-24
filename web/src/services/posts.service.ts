/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { signal } from '@preact/signals';
import { Errors } from '../models/errors.ts';
import { Post } from '../models/post.ts';
import { $authToken, $authUser } from './auth.service.ts';

export const $addPost = signal<Post | null>(null);

export class PostsService {
    static instance?: PostsService;

    static getInstance(): PostsService {
        if (PostsService.instance === undefined) {
            PostsService.instance = new PostsService();
        }
        return PostsService.instance;
    }

    async getAll(page: number): Promise<Post[]> {
        const headers = new Headers();
        if ($authUser.value !== null) {
            headers.append('Authorization', `Bearer ${$authToken.value}`);
        }
        const res = await fetch(`${import.meta.env.VITE_API_URL}/posts?page=${page}`, { headers });
        return (await res.json()) as Post[];
    }

    async get(id: string): Promise<Post | null> {
        const headers = new Headers();
        if ($authUser.value !== null) {
            headers.append('Authorization', `Bearer ${$authToken.value}`);
        }
        const res = await fetch(`${import.meta.env.VITE_API_URL}/posts/${id}`, { headers });
        if (res.status === 404) {
            return null;
        }
        return (await res.json()) as Post;
    }

    async create(text: string): Promise<[boolean, Post | Errors]> {
        const res = await fetch(`${import.meta.env.VITE_API_URL}/posts`, {
            method: 'POST',
            headers: {
                Authorization: `Bearer ${$authToken.value}`,
            },
            body: new URLSearchParams({ text }),
        });
        if (res.status != 200) {
            return [false, (await res.json()) as Errors];
        }
        return [true, (await res.json()) as Post];
    }

    async update(id: string, text: string): Promise<[boolean, Post | Errors]> {
        const res = await fetch(`${import.meta.env.VITE_API_URL}/posts/${id}`, {
            method: 'PUT',
            headers: {
                Authorization: `Bearer ${$authToken.value}`,
            },
            body: new URLSearchParams({ text }),
        });
        if (res.status == 200) {
            return [true, (await res.json()) as Post];
        }
        return [false, (await res.json()) as Errors];
    }

    async delete(id: string): Promise<void> {
        await fetch(`${import.meta.env.VITE_API_URL}/posts/${id}`, {
            method: 'DELETE',
            headers: {
                Authorization: `Bearer ${$authToken.value}`,
            },
        });
    }

    async reply(parent_post_id: string, text: string): Promise<[boolean, Post | Errors]> {
        const res = await fetch(`${import.meta.env.VITE_API_URL}/posts/${parent_post_id}/reply`, {
            method: 'POST',
            headers: {
                Authorization: `Bearer ${$authToken.value}`,
            },
            body: new URLSearchParams({ text }),
        });
        if (res.status == 200) {
            return [true, (await res.json()) as Post];
        }
        return [false, (await res.json()) as Errors];
    }

    async repost(id: string): Promise<Post | null> {
        const res = await fetch(`${import.meta.env.VITE_API_URL}/posts/${id}/repost`, {
            method: 'POST',
            headers: {
                Authorization: `Bearer ${$authToken.value}`,
            },
        });
        if (res.status == 200) {
            return (await res.json()) as Post;
        }
        return null;
    }

    async like(id: string): Promise<void> {
        await fetch(`${import.meta.env.VITE_API_URL}/posts/${id}/like`, {
            method: 'PUT',
            headers: {
                Authorization: `Bearer ${$authToken.value}`,
            },
        });
    }

    async remove_like(id: string): Promise<void> {
        await fetch(`${import.meta.env.VITE_API_URL}/posts/${id}/like`, {
            method: 'DELETE',
            headers: {
                Authorization: `Bearer ${$authToken.value}`,
            },
        });
    }

    async dislike(id: string): Promise<void> {
        await fetch(`${import.meta.env.VITE_API_URL}/posts/${id}/dislike`, {
            method: 'PUT',
            headers: {
                Authorization: `Bearer ${$authToken.value}`,
            },
        });
    }

    async remove_dislike(id: string): Promise<void> {
        await fetch(`${import.meta.env.VITE_API_URL}/posts/${id}/dislike`, {
            method: 'DELETE',
            headers: {
                Authorization: `Bearer ${$authToken.value}`,
            },
        });
    }
}
