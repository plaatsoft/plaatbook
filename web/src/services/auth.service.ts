/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { signal } from '@preact/signals';
import { Session } from '../models/session.ts';
import { User } from '../models/user.ts';
import { Errors } from '../models/errors.ts';

export const $authSession = signal<(Session | null) | undefined>(undefined);
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

        // Save token
        const { token, session, user } = (await res.json()) as { token: string; session: Session; user: User };
        localStorage.setItem('token', token);
        $authSession.value = session;
        $authUser.value = user;
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
        // Get token
        const token = localStorage.getItem('token');
        if (token === null) {
            $authSession.value = null;
            $authUser.value = null;
            return 'not_authed';
        }

        // Try to get user
        const res = await fetch(`${import.meta.env.VITE_API_URL}/auth/validate`, {
            headers: {
                Authorization: `Bearer ${localStorage.getItem('token')}`,
            },
        });
        if (res.status != 200) {
            AuthService.getInstance().logout();
            return 'logout';
        }
        const { session, user } = (await res.json()) as { session: Session; user: User };
        $authSession.value = session;
        $authUser.value = user;
        return 'authed';
    }

    async logout(): Promise<boolean> {
        // Try to logout current token
        await fetch(`${import.meta.env.VITE_API_URL}/auth/logout`, {
            method: 'PUT',
            headers: {
                Authorization: `Bearer ${localStorage.getItem('token')}`,
            },
        });

        // Clear stores
        localStorage.removeItem('token');
        $authSession.value = null;
        $authUser.value = null;
        return true;
    }

    async changeDetails(username: string, email: string): Promise<Errors | undefined> {
        // Try to change user details
        const res = await fetch(`${import.meta.env.VITE_API_URL}/users/${$authUser.value!.id}`, {
            method: 'PUT',
            headers: {
                Authorization: `Bearer ${localStorage.getItem('token')}`,
            },
            body: new URLSearchParams({
                username,
                email,
            }),
        });
        if (res.status != 200) {
            return (await res.json()) as Errors;
        }
        $authUser.value = {
            ...$authUser.value!,
            username,
            email,
        };
        return undefined;
    }

    async changePassword(current_password: string, password: string): Promise<Errors | undefined> {
        // Try to change user password
        const res = await fetch(`${import.meta.env.VITE_API_URL}/users/${$authUser.value!.id}/change_password`, {
            method: 'PUT',
            headers: {
                Authorization: `Bearer ${localStorage.getItem('token')}`,
            },
            body: new URLSearchParams({
                current_password,
                password,
            }),
        });
        if (res.status != 200) {
            return (await res.json()) as Errors;
        }
        return undefined;
    }
}
