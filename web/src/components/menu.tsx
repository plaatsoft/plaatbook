/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useState, useEffect } from 'preact/hooks';
// eslint-disable-next-line import/named
import { RouterOnChangeArgs } from 'preact-router';
import { AuthService, $authUser } from '../services/auth.service.ts';
import { AppIcon, LogoutIcon, SearchIcon, SettingsIcon } from './icons.tsx';

const styles = css`
    .avatar {
        height: var(--bulma-navbar-item-img-max-height);
    }
`;

export function Menu({ route }: { route: RouterOnChangeArgs | null }) {
    const [isOpen, setIsOpen] = useState(false);

    useEffect(() => {
        setIsOpen(false);
    }, [route]);

    const logout = async (event: MouseEvent) => {
        event.preventDefault();
        await AuthService.getInstance().logout();
    };

    return (
        <div className="navbar is-fixed-top">
            <div className="navbar-brand has-text-weight-bold">
                <a className="navbar-item" href="/">
                    <AppIcon />
                    PlaatBook
                </a>

                <a className="navbar-burger" onClick={() => setIsOpen(!isOpen)}>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                </a>
            </div>
            <div className={`navbar-menu ${isOpen ? 'is-active' : ''}`}>
                <div className="navbar-start">
                    <a className="navbar-item" href="/search">
                        <SearchIcon />
                        Search
                    </a>
                </div>

                <div className="navbar-end">
                    {$authUser.value && (
                        <>
                            <div className="navbar-item has-dropdown is-hoverable">
                                <a className="navbar-link" href={`/users/${$authUser.value.username}`}>
                                    <img className={styles.avatar} src="/images/avatar.svg" />
                                    <strong>@{$authUser.value.username}</strong>
                                </a>
                                <div className="navbar-dropdown">
                                    <a className="navbar-item" href="/settings">
                                        <SettingsIcon className="is-small" />
                                        Settings
                                    </a>
                                    <a className="navbar-item" href="#" onClick={logout}>
                                        <LogoutIcon className="is-small" />
                                        Logout
                                    </a>
                                </div>
                            </div>
                        </>
                    )}

                    {$authUser.value === null && (
                        <div className="navbar-item">
                            <div className="buttons">
                                <a className="button is-link" href="/auth/login">
                                    Login
                                </a>
                                <a className="button" href="/auth/register">
                                    Register
                                </a>
                            </div>
                        </div>
                    )}
                </div>
            </div>
        </div>
    );
}
