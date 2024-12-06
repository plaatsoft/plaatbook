/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useLocation } from 'preact-iso';
import { AuthService, $authUser } from '../services/auth.service.ts';

export function Menu() {
    const location = useLocation();

    const logout = async () => {
        if (await AuthService.getInstance().logout()) {
            location.route('/auth/login');
        }
    };

    return (
        <div className="navbar">
            <div className="navbar-brand has-text-weight-bold">
                <a className="navbar-item" href="/">
                    PlaatBook
                </a>
            </div>
            <div className="navbar-menu">
                <div className="navbar-start">
                    <a className="navbar-item" href="/about">
                        About
                    </a>
                </div>

                <div className="navbar-end">
                    {$authUser.value !== null && $authUser.value !== undefined && (
                        <>
                            <div className="navbar-item">{$authUser.value.username}</div>
                            <div className="navbar-item">
                                <div className="buttons">
                                    <a className="button is-link" href="#" onClick={logout}>
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
