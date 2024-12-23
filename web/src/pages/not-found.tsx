/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useEffect } from 'preact/hooks';

export function NotFound() {
    useEffect(() => {
        document.title = '404 Not Found - PlaatBook';
    }, []);

    return (
        <div className="section">
            <h2 className="title">404 Not Found</h2>
        </div>
    );
}
