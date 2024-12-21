/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useEffect, useState } from 'preact/hooks';
import { PostsService } from '../../services/posts.service.ts';
import { PostComponent } from '../../components/post.tsx';
import { NotFound } from '../not-found.tsx';
import { Post } from '../../models/post.ts';
import { useLocation } from 'preact-iso';

export function PostsShow({ post_id }: { post_id: string }) {
    const location = useLocation();
    const [post, setPost] = useState<Post | null | undefined>(undefined);

    const getPost = async () => {
        setPost(await PostsService.getInstance().get(post_id));
    };

    useEffect(() => {
        getPost();
    }, [post_id]);

    return (
        <>
            {post !== null && post !== undefined && (
                <div className="section">
                    <PostComponent
                        post={post}
                        onUpdate={(post) => setPost(post)}
                        onDelete={() => location.route('/')}
                        isFullPage
                    />
                </div>
            )}

            {post === null && <NotFound />}
        </>
    );
}
