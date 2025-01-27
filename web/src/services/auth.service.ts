/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { signal } from '@preact/signals';
import { Session, Report, User, AuthLoginResponse, AuthValidateResponse, SessionIndexResponse } from '../api.ts';
import { route } from '../router.tsx';

export const $authToken = signal<(string | null) | undefined>(undefined);
export const $authSession = signal<(Session | null) | undefined>(undefined);
export const $authUser = signal<(User | null) | undefined>(undefined);
export const $authUsers = signal<(User[] | null) | undefined>(undefined);

export class AuthService {
    static instance?: AuthService;

    static getInstance(): AuthService {
        if (AuthService.instance === undefined) {
            AuthService.instance = new AuthService();
        }
        return AuthService.instance;
    }

    async login(logon: string, password: string): Promise<boolean> {
        // Check if user is already logged in
        if ($authUsers.value) {
            for (let i = 0; i < $authUsers.value.length; i++) {
                if ($authUsers.value[i].username === logon || $authUsers.value[i].email === logon) {
                    await this.selectToken(i);
                    return true;
                }
            }
        }

        // Login
        const res = await fetch(`${import.meta.env.VITE_API_URL}/auth/login`, {
            method: 'POST',
            body: new URLSearchParams({ logon, password }),
        });
        if (res.status != 200) {
            return false;
        }
        const { token } = (await res.json()) as AuthLoginResponse;

        // Update stores
        const tokens = JSON.parse(localStorage.getItem('tokens') || '[]') as string[];
        tokens.unshift(token);
        localStorage.setItem('tokens', JSON.stringify(tokens));
        await this.updateAuth();
        return true;
    }

    async register(username: string, email: string, password: string): Promise<Report | null> {
        const res = await fetch(`${import.meta.env.VITE_API_URL}/users`, {
            method: 'POST',
            body: new URLSearchParams({ username, email, password }),
        });
        if (res.status != 200) {
            return (await res.json()) as Report;
        }
        return null;
    }

    async updateAuth(): Promise<void> {
        const tokens = JSON.parse(localStorage.getItem('tokens') || '[]') as string[];
        if (tokens.length === 0) {
            $authToken.value = null;
            $authSession.value = null;
            $authUser.value = null;
            $authUsers.value = null;
            return;
        }

        const authUsers = [];
        for (let i = 0; i < tokens.length; i++) {
            const token = tokens[i];

            // Validate token
            const res = await fetch(`${import.meta.env.VITE_API_URL}/auth/validate`, {
                headers: {
                    Authorization: `Bearer ${token}`,
                },
            });
            if (res.status != 200) {
                const tokens = (JSON.parse(localStorage.getItem('tokens') || '[]') as string[]).filter(
                    (t) => t !== token,
                );
                localStorage.setItem('tokens', JSON.stringify(tokens));
                continue;
            }

            const { session, user } = (await res.json()) as AuthValidateResponse;
            if (i == 0) {
                $authToken.value = token;
                $authSession.value = session;
                $authUser.value = user;
            }
            authUsers.push(user);
        }
        $authUsers.value = authUsers;
    }

    async selectToken(index: number): Promise<void> {
        const tokens = JSON.parse(localStorage.getItem('tokens') || '[]') as string[];
        const [selectedToken] = tokens.splice(index, 1);
        tokens.unshift(selectedToken);
        localStorage.setItem('tokens', JSON.stringify(tokens));
        await this.updateAuth();
    }

    async logout(): Promise<boolean> {
        await fetch(`${import.meta.env.VITE_API_URL}/auth/logout`, {
            method: 'PUT',
            headers: {
                Authorization: `Bearer ${$authToken.value}`,
            },
        });

        // Clear stores
        const tokens = (JSON.parse(localStorage.getItem('tokens') || '[]') as string[]).filter(
            (t) => t !== $authToken.value,
        );
        localStorage.setItem('tokens', JSON.stringify(tokens));
        await this.updateAuth();
        if ($authToken.value === null) route('/auth/login');
        return true;
    }

    async getActiveSessions(page: number): Promise<SessionIndexResponse | null> {
        const res = await fetch(`${import.meta.env.VITE_API_URL}/users/${$authUser.value!.id}/sessions?page=${page}`, {
            headers: {
                Authorization: `Bearer ${$authToken.value}`,
            },
        });
        if (res.status != 200) {
            return null;
        }
        return (await res.json()) as SessionIndexResponse;
    }

    async revokeSession(session: Session): Promise<boolean> {
        if (session.id === $authSession.value!.id) {
            return this.logout();
        }
        const res = await fetch(`${import.meta.env.VITE_API_URL}/sessions/${session.id}`, {
            method: 'DELETE',
            headers: {
                Authorization: `Bearer ${$authToken.value}`,
            },
        });
        if (res.status != 200) {
            return false;
        }
        return true;
    }

    async changePassword(current_password: string, password: string): Promise<Report | null> {
        const res = await fetch(`${import.meta.env.VITE_API_URL}/users/${$authUser.value!.id}/change_password`, {
            method: 'PUT',
            headers: {
                Authorization: `Bearer ${$authToken.value}`,
            },
            body: new URLSearchParams({ current_password, password }),
        });
        if (res.status != 200) {
            return (await res.json()) as Report;
        }
        return null;
    }
}
