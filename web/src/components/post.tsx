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

    const likePost = async () => {
        if (!post.auth_user_liked) {
            await PostsService.getInstance().like(post.id);
        } else {
            await PostsService.getInstance().remove_like(post.id);
        }
        refreshPosts$.value = Date.now();
    };

    const dislikePost = async () => {
        if (!post.auth_user_disliked) {
            await PostsService.getInstance().dislike(post.id);
        } else {
            await PostsService.getInstance().remove_dislike(post.id);
        }
        refreshPosts$.value = Date.now();
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
                {$authUser.value && (
                    <div className="buttons mt-2">
                        <button
                            className={`button is-small ${post.auth_user_liked ? 'is-link' : ''} pl-3`}
                            onClick={likePost}
                        >
                            <svg className="icon mr-1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                                <path d="M23,10C23,8.89 22.1,8 21,8H14.68L15.64,3.43C15.66,3.33 15.67,3.22 15.67,3.11C15.67,2.7 15.5,2.32 15.23,2.05L14.17,1L7.59,7.58C7.22,7.95 7,8.45 7,9V19A2,2 0 0,0 9,21H18C18.83,21 19.54,20.5 19.84,19.78L22.86,12.73C22.95,12.5 23,12.26 23,12V10M1,21H5V9H1V21Z" />
                            </svg>
                            {post.likes}
                        </button>
                        <button
                            className={`button is-small ${post.auth_user_disliked ? 'is-danger' : ''} pl-3`}
                            onClick={dislikePost}
                        >
                            <svg className="icon mr-1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                                <path d="M19,15H23V3H19M15,3H6C5.17,3 4.46,3.5 4.16,4.22L1.14,11.27C1.05,11.5 1,11.74 1,12V14A2,2 0 0,0 3,16H9.31L8.36,20.57C8.34,20.67 8.33,20.77 8.33,20.88C8.33,21.3 8.5,21.67 8.77,21.94L9.83,23L16.41,16.41C16.78,16.05 17,15.55 17,15V5C17,3.89 16.1,3 15,3Z" />
                            </svg>
                            {post.dislikes}
                        </button>
                    </div>
                )}
            </div>
        </div>
    );
}
