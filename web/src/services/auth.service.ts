/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { signal } from '@preact/signals';
import { User } from '../models/user.ts';

export type Errors = Record<string, string[]>;

export const $authUser = signal<(User | null) | undefined>(undefined);

export class AuthService {
    static instance?: AuthService;

    static getInstance(): AuthService {
        if (AuthService.instance === undefined) {
            AuthService.instance = new AuthService();
        }
        return AuthService.instance;
    }

    async login(logon: string, password: string): Promise<boolean> {
        // Try to login with logon and password
        const res = await fetch(`${import.meta.env.VITE_API_URL}/auth/login`, {
            method: 'POST',
            body: new URLSearchParams({
                logon,
                password,
            }),
        });
        if (res.status != 200) {
            return false;
        }

        // Save user_id and token
        const { token, user } = (await res.json()) as { token: string; user: User };
        $authUser.value = user;
        localStorage.setItem('token', token);
        localStorage.setItem('user_id', user.id);
        return true;
    }

    async register(username: string, email: string, password: string): Promise<Errors | undefined> {
        // Try to register with username, email and password
        const res = await fetch(`${import.meta.env.VITE_API_URL}/users`, {
            method: 'POST',
            body: new URLSearchParams({
                username,
                email,
                password,
            }),
        });
        if (res.status != 200) {
            return (await res.json()) as Errors;
        }
        return undefined;
    }

    async auth(): Promise<string> {
        // Get token and user_id
        const token = localStorage.getItem('token');
        const user_id = localStorage.getItem('user_id');
        if (token === null) {
            $authUser.value = null;
            return 'not_authed';
        }

        // Try to get user
        const res = await fetch(`${import.meta.env.VITE_API_URL}/users/${user_id}`, {
            headers: {
                Authorization: `Bearer ${localStorage.getItem('token')}`,
            },
        });
        if (res.status != 200) {
            AuthService.getInstance().logout();
            return 'logout';
        }
        $authUser.value = await res.json();
        return 'authed';
    }

    async logout(): Promise<boolean> {
        // Try to logout current token
        const res = await fetch(`${import.meta.env.VITE_API_URL}/auth/logout`, {
            method: 'POST',
            headers: {
                Authorization: `Bearer ${localStorage.getItem('token')}`,
            },
        });
        if (res.status != 200) {
            return false;
        }

        // Clear stores
        $authUser.value = null;

        // Remove user_id and token
        localStorage.removeItem('token');
        localStorage.removeItem('user_id');
        return true;
    }
}
