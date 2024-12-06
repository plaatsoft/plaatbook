/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { $authUser } from '../services/auth.service.ts';

export function Home() {
    return (
        <div className="section">
            {$authUser.value !== null ? (
                <h2 className="title">Welcome {$authUser.value?.username}!</h2>
            ) : (
                <h2 className="title">Home</h2>
            )}
        </div>
    );
}
