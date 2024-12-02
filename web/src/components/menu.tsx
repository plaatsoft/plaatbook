/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

export function Menu() {
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
            </div>
        </div>
    );
}
