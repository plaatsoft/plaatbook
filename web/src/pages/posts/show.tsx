/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useEffect, useState } from 'preact/hooks';
import { route } from 'preact-router';
import { PostsService } from '../../services/posts.service.ts';
import { PostComponent } from '../../components/post.tsx';
import { NotFound } from '../not-found.tsx';
import { Post } from '../../models/post.ts';

export function PostsShow({ post_id }: { post_id: string }) {
    const [post, setPost] = useState<Post | null | undefined>(undefined);

    const getPost = async () => {
        setPost(await PostsService.getInstance().get(post_id));
    };
    useEffect(() => {
        getPost();
    }, [post_id]);

    useEffect(() => {
        if (post) document.title = `Post by ${post!.user!.username} - PlaatBook`;
    }, [post]);

    return (
        <>
            {post !== null && post !== undefined && (
                <div className="section">
                    <PostComponent
                        post={post}
                        onUpdate={(updatedPost) => {
                            if (updatedPost) setPost(updatedPost);
                            else route('/');
                        }}
                        isFullPage
                    />
                </div>
            )}

            {post === null && <NotFound />}
        </>
    );
}
