/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useState, useEffect } from 'preact/hooks';
import { AuthService, $authUser, $authUsers } from '../services/auth.service.ts';
import { AccountIcon, AppIcon, LoginIcon, LogoutIcon, RegisterIcon, SearchIcon, SettingsIcon } from './icons.tsx';
import { DialogService } from '../services/dialog.service.tsx';
import { LoginDialog } from './dialogs/login-dialog.tsx';
import { $route, Link, route } from '../router.tsx';

const styles = css`
    .avatar {
        height: var(--bulma-navbar-item-img-max-height);
    }
`;

export function Menu() {
    const [isOpen, setIsOpen] = useState(false);
    const [isAuthOpen, setIsAuthOpen] = useState(false);

    useEffect(() => {
        setIsOpen(false);
        setIsAuthOpen(false);
    }, [$route.value]);

    const selectToken = async (event: MouseEvent, index: number) => {
        event.preventDefault();
        await AuthService.getInstance().selectToken(index);
    };

    const addAccount = async (event: MouseEvent) => {
        event.preventDefault();
        if (await DialogService.getInstance().open<boolean>(LoginDialog)) {
            route('/');
        }
    };

    const logout = async (event: MouseEvent) => {
        event.preventDefault();
        await AuthService.getInstance().logout();
    };

    return (
        <div className="navbar is-fixed-top">
            <div className="navbar-brand has-text-weight-bold">
                <Link className="navbar-item" href="/">
                    <AppIcon />
                    PlaatBook
                </Link>

                <button className="navbar-burger" onClick={() => setIsOpen(!isOpen)}>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                </button>
            </div>
            <div className={`navbar-menu ${isOpen ? 'is-active' : ''}`}>
                <div className="navbar-start">
                    <Link className="navbar-item" href="/search">
                        <SearchIcon />
                        Search
                    </Link>
                    {$authUser.value && (
                        <Link className="navbar-item" href={`/users/${$authUser.value.username}`}>
                            <AccountIcon />
                            Profile
                        </Link>
                    )}
                </div>

                <div className="navbar-end">
                    {$authUsers.value && (
                        <>
                            <div
                                className={`navbar-item has-dropdown ${isAuthOpen ? 'is-active' : ''}`}
                                onClick={() => setIsAuthOpen(!isAuthOpen)}
                            >
                                <div className="navbar-link">
                                    <img className={styles.avatar} src="/images/avatar.svg" />
                                    <strong>@{$authUsers.value[0].username}</strong>
                                </div>

                                <div className="navbar-dropdown">
                                    {$authUsers.value.slice(1).map((user, index) => (
                                        <a
                                            className="navbar-item"
                                            href="#"
                                            onClick={(e) => selectToken(e, index + 1)}
                                            style="height: 52px;"
                                            key={user.id}
                                        >
                                            <img className={styles.avatar} src="/images/avatar.svg" />@{user.username}
                                        </a>
                                    ))}
                                    <a className="navbar-item" href="#" onClick={addAccount}>
                                        <LoginIcon className="is-small" />
                                        Add account
                                    </a>
                                    <hr className="navbar-divider" />

                                    <Link className="navbar-item" href="/settings">
                                        <SettingsIcon className="is-small" />
                                        Settings
                                    </Link>
                                    <a className="navbar-item" href="#" onClick={logout}>
                                        <LogoutIcon className="is-small" />
                                        Logout
                                    </a>
                                </div>
                            </div>
                        </>
                    )}

                    {$authUser.value === null && (
                        <>
                            <Link className="navbar-item" href="/auth/login">
                                <LoginIcon />
                                Login
                            </Link>
                            <Link className="navbar-item" href="/auth/register">
                                <RegisterIcon />
                                Register
                            </Link>
                        </>
                    )}
                </div>
            </div>
        </div>
    );
}
