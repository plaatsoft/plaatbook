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
                    <svg className="icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                        <path d="M18 2H12V9L9.5 7.5L7 9V2H6A2 2 0 0 0 4 4V20A2 2 0 0 0 6 22H18A2 2 0 0 0 20 20V4A2 2 0 0 0 18 2M14 12A2 2 0 1 1 12 14A2 2 0 0 1 14 12M18 20H10V19C10 17.67 12.67 17 14 17S18 17.67 18 19Z" />
                    </svg>
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
                        <svg className="icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                            <path d="M9.5,3A6.5,6.5 0 0,1 16,9.5C16,11.11 15.41,12.59 14.44,13.73L14.71,14H15.5L20.5,19L19,20.5L14,15.5V14.71L13.73,14.44C12.59,15.41 11.11,16 9.5,16A6.5,6.5 0 0,1 3,9.5A6.5,6.5 0 0,1 9.5,3M9.5,5C7,5 5,7 5,9.5C5,12 7,14 9.5,14C12,14 14,12 14,9.5C14,7 12,5 9.5,5Z" />
                        </svg>
                        Search
                    </a>
                </div>

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
