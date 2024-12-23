/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useState, useEffect } from 'preact/hooks';
import { $authUser } from '../services/auth.service.ts';
import { PostsService, $addPost } from '../services/posts.service.ts';
import { Post } from '../models/post.ts';
import { PostComponent } from '../components/post.tsx';
import { PostCreateForm } from '../components/post-create-form.tsx';
import { InfiniteList } from '../components/infinite-list.tsx';

export function Home() {
    return (
        <div className="section">
            {$authUser.value !== null ? (
                <h2 className="title">Welcome {$authUser.value?.username}!</h2>
            ) : (
                <h2 className="title">Home</h2>
            )}

            {$authUser.value !== null ? <PostCreateForm /> : null}

            <PostsList />
        </div>
    );
}

function PostsList() {
    const [posts, setPosts] = useState<Post[]>([]);

    useEffect(() => {
        if ($addPost.value) posts.unshift($addPost.value);
        $addPost.value = null;
    }, [$addPost.value]);

    return (
        <InfiniteList
            items={posts}
            fetchPage={async (page) => {
                const newPosts = await PostsService.getInstance().getAll(page);
                setPosts((posts) => [...posts, ...newPosts]);
            }}
            template={(post) => (
                <PostComponent
                    post={post}
                    onUpdate={(updatedPost) => {
                        if (updatedPost) setPosts((posts) => posts.map((p) => (p.id === post.id ? updatedPost : p)));
                        else setPosts((posts) => posts.filter((p) => p.id !== post.id));
                    }}
                    key={post.id}
                />
            )}
        />
    );
}
