/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { User } from '../models/user.ts';
import { dateFormatAgo } from '../utils.ts';

export function UserComponent({ user }: { user: User }) {
    return (
        <div className="media">
            <div className="media-left">
                <img className="image is-64x64" src="/images/avatar.svg" />
            </div>
            <div className="media-content">
                <p>
                    <a href={`/users/${user.username}`}>
                        <strong>@{user.username}</strong>
                    </a>
                    <br />
                    Joined {dateFormatAgo(user.created_at)}
                </p>
            </div>
        </div>
    );
}
