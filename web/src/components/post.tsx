/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { Post } from '../models/post.ts';
import { $authUser } from '../services/auth.service.ts';
import { DialogService } from '../services/dialog.service.tsx';
import { PostsService, refreshPosts$ } from '../services/posts.service.ts';
import { dateFormatAgo } from '../utils.ts';
import { PostEditModal } from './modals/post-edit-modal.tsx';

export function PostComponent({ post }: { post: Post }) {
    const editPost = async () => {
        if (await DialogService.getInstance().open<Post | null>(PostEditModal, { post })) {
            refreshPosts$.value = Date.now();
        }
    };

    const deletePost = async () => {
        if (
            await DialogService.getInstance().confirm(
                'Are you sure?',
                'Are you sure you want to delete this post?',
                'Delete',
            )
        ) {
            await PostsService.getInstance().delete(post.id);
            refreshPosts$.value = Date.now();
        }
    };

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
                    <small>
                        {dateFormatAgo(post.created_at)}
                        {post.created_at !== post.updated_at && <span className="tag ml-2">Edited</span>}
                    </small>
                    {$authUser.value && $authUser.value.id === post.user!.id && (
                        <>
                            <button className="button is-small ml-2" onClick={editPost}>
                                <svg className="icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                                    <path d="M20.71,7.04C21.1,6.65 21.1,6 20.71,5.63L18.37,3.29C18,2.9 17.35,2.9 16.96,3.29L15.12,5.12L18.87,8.87M3,17.25V21H6.75L17.81,9.93L14.06,6.18L3,17.25Z" />
                                </svg>
                            </button>

                            <button className="button is-small ml-2" onClick={deletePost}>
                                <svg className="icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                                    <path d="M19,4H15.5L14.5,3H9.5L8.5,4H5V6H19M6,19A2,2 0 0,0 8,21H16A2,2 0 0,0 18,19V7H6V19Z" />
                                </svg>
                            </button>
                        </>
                    )}
                </p>
                <p>{post.text}</p>
            </div>
        </div>
    );
}
