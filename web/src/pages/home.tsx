/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useState, useEffect } from 'preact/hooks';
import { $authUser } from '../services/auth.service.ts';
import { PostsService, $refreshPosts } from '../services/posts.service.ts';
import { Post } from '../models/post.ts';
import { PostComponent } from '../components/post.tsx';
import { CreatePost } from '../components/create-post.tsx';

export function Home() {
    return (
        <div className="section">
            {$authUser.value !== null ? (
                <h2 className="title">Welcome {$authUser.value?.username}!</h2>
            ) : (
                <h2 className="title">Home</h2>
            )}

            {$authUser.value !== null ? <CreatePost /> : null}

            <PostsList />
        </div>
    );
}

function PostsList() {
    const [posts, setPosts] = useState<Post[]>([]);

    const fetchPosts = async () => {
        setPosts(await PostsService.getInstance().getAll());
    };
    useEffect(() => {
        fetchPosts();
    }, [$refreshPosts.value]);

    return (
        <>
            {posts.map((post) => (
                <PostComponent post={post} key={post.id} />
            ))}
        </>
    );
}
