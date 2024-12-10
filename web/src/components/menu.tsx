/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useState, useEffect } from 'preact/hooks';
import { useLocation } from 'preact-iso';
import { AuthService, $authUser } from '../services/auth.service.ts';

export function Menu() {
    const location = useLocation();
    const [isOpen, setIsOpen] = useState(false);

    useEffect(() => {
        setIsOpen(false);
    }, [location]);

    const logout = async (event: MouseEvent) => {
        event.preventDefault();
        await AuthService.getInstance().logout(location);
    };

    return (
        <div className="navbar is-fixed-top">
            <div className="navbar-brand has-text-weight-bold">
                <a className="navbar-item" href="/">
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
                {/* <div className="navbar-start">
                    <a className="navbar-item" href="/about">
                        About
                    </a>
                </div> */}

                <div className="navbar-end">
                    {$authUser.value !== null && $authUser.value !== undefined && (
                        <>
                            <a className="navbar-item" href={`/users/${$authUser.value.username}`}>
                                <strong>@{$authUser.value.username}</strong>
                            </a>
                            <div className="navbar-item">
                                <div className="buttons">
                                    <a className="button" href="/settings">
                                        Settings
                                    </a>
                                    <a className="button" href="#" onClick={logout}>
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
