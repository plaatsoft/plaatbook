/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { Post } from '../models/post.ts';
import { dateFormatAgo } from '../utils.ts';

export function PostComponent({ post }: { post: Post }) {
    return (
        <div className="media">
            <div className="media-left">
                <img className="image is-64x64" src="/images/avatar.svg" />
            </div>
            <div className="media-content">
                <p>
                    <a href={`/users/${post.user!.username}`}>
                        <strong>@{post.user!.username}</strong>
                    </a>{' '}
                    <small>{dateFormatAgo(post.created_at)}</small>
                    <br /> {post.text}
                </p>
            </div>
        </div>
    );
}
