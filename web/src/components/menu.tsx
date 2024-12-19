/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useState, useEffect } from 'preact/hooks';
import { useLocation } from 'preact-iso';
import { AuthService, $authUser } from '../services/auth.service.ts';

const styles = css`
    .avatar {
        height: var(--bulma-navbar-item-img-max-height);
    }
`;

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
                    {$authUser.value && (
                        <>
                            <div className="navbar-item has-dropdown is-hoverable">
                                <a className="navbar-link" href={`/users/${$authUser.value.username}`}>
                                    <img className={styles.avatar} src="/images/avatar.svg" />
                                    <strong>@{$authUser.value.username}</strong>
                                </a>
                                <div className="navbar-dropdown">
                                    <a className="navbar-item" href="/settings">
                                        <svg
                                            className="icon is-small"
                                            xmlns="http://www.w3.org/2000/svg"
                                            viewBox="0 0 24 24"
                                        >
                                            <path d="M12,15.5A3.5,3.5 0 0,1 8.5,12A3.5,3.5 0 0,1 12,8.5A3.5,3.5 0 0,1 15.5,12A3.5,3.5 0 0,1 12,15.5M19.43,12.97C19.47,12.65 19.5,12.33 19.5,12C19.5,11.67 19.47,11.34 19.43,11L21.54,9.37C21.73,9.22 21.78,8.95 21.66,8.73L19.66,5.27C19.54,5.05 19.27,4.96 19.05,5.05L16.56,6.05C16.04,5.66 15.5,5.32 14.87,5.07L14.5,2.42C14.46,2.18 14.25,2 14,2H10C9.75,2 9.54,2.18 9.5,2.42L9.13,5.07C8.5,5.32 7.96,5.66 7.44,6.05L4.95,5.05C4.73,4.96 4.46,5.05 4.34,5.27L2.34,8.73C2.21,8.95 2.27,9.22 2.46,9.37L4.57,11C4.53,11.34 4.5,11.67 4.5,12C4.5,12.33 4.53,12.65 4.57,12.97L2.46,14.63C2.27,14.78 2.21,15.05 2.34,15.27L4.34,18.73C4.46,18.95 4.73,19.03 4.95,18.95L7.44,17.94C7.96,18.34 8.5,18.68 9.13,18.93L9.5,21.58C9.54,21.82 9.75,22 10,22H14C14.25,22 14.46,21.82 14.5,21.58L14.87,18.93C15.5,18.67 16.04,18.34 16.56,17.94L19.05,18.95C19.27,19.03 19.54,18.95 19.66,18.73L21.66,15.27C21.78,15.05 21.73,14.78 21.54,14.63L19.43,12.97Z" />
                                        </svg>
                                        Settings
                                    </a>
                                    <a className="navbar-item" href="#" onClick={logout}>
                                        {' '}
                                        <svg
                                            className="icon is-small"
                                            xmlns="http://www.w3.org/2000/svg"
                                            viewBox="0 0 24 24"
                                        >
                                            <path d="M17 7L15.59 8.41L18.17 11H8V13H18.17L15.59 15.58L17 17L22 12M4 5H12V3H4C2.9 3 2 3.9 2 5V19C2 20.1 2.9 21 4 21H12V19H4V5Z" />
                                        </svg>
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
