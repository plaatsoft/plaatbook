/*
 * Copyright (c) 2024-2025 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { Post, PostType } from '../api.ts';
import { $authUser } from '../services/auth.service.ts';
import { DialogService } from '../services/dialog.service.tsx';
import { PostsService } from '../services/posts.service.ts';
import { dateFormatAgo } from '../utils.ts';
import {
    CommentIcon,
    DeleteIcon,
    DislikeIcon,
    EditIcon,
    LikeIcon,
    OptionsIcon,
    RepostIcon,
    ShareIcon,
    StatsIcon,
} from './icons.tsx';
import { PostEditDialog } from './dialogs/post-edit-dialog.tsx';
import { PostReplyDialog } from './dialogs/post-reply-dialog.tsx';
import { PostShareDialog } from './dialogs/post-share-dialog.tsx';
import { Link, route } from '../router.tsx';

export function PostComponent({
    post,
    onUpdate,
    isFullPage,
    replyHideParent,
}: {
    post: Post;
    onUpdate?: (post: Post | null) => void;
    isFullPage?: boolean;
    replyHideParent?: boolean;
}) {
    const openPost = (event: MouseEvent) => {
        event.stopPropagation();
        route(`/posts/${post.id}`);
    };

    const contentPost = post.type === PostType.REPOST ? post.parentPost! : post;
    return (
        <>
            {isFullPage && contentPost.type == PostType.REPLY && (
                <PostComponent post={contentPost.parentPost!} isFullPage={isFullPage} replyHideParent />
            )}

            <div className="media" onClick={openPost}>
                <div className="media-left">
                    <Link href={`/users/${contentPost.user!.username}`}>
                        <img className="image is-64x64" src="/images/avatar.svg" />
                    </Link>
                </div>

                <div className="media-content">
                    {post.type === PostType.REPLY && (
                        <div className="mb-1">
                            <Link href={`/posts/${post.parentPost!.id}`} style="color: inherit;">
                                <CommentIcon className="is-small mr-1" />
                                {post.user!.username} replied
                            </Link>
                        </div>
                    )}
                    {post.type === PostType.REPOST && (
                        <div className="mb-1">
                            <Link href={`/posts/${post.parentPost!.id}`} style="color: inherit;">
                                <RepostIcon className="is-small mr-1" />
                                {post.user!.username} reposted
                            </Link>
                        </div>
                    )}

                    <div className="mb-1" style="position: relative;">
                        <Link className="mr-2" href={`/users/${contentPost.user!.username}`}>
                            <strong>@{contentPost.user!.username}</strong>
                        </Link>
                        <small className="mr-2">{dateFormatAgo(contentPost.createdAt)}</small>
                        {contentPost.createdAt !== contentPost.updatedAt && (
                            <span className="tag" style="position: absolute; top: 0;">
                                <EditIcon className="is-small mr-1" />
                                Edited
                            </span>
                        )}

                        {$authUser.value && $authUser.value.id === post.user!.id && (
                            <PostOptions post={post} onUpdate={onUpdate} />
                        )}
                    </div>

                    <div className="content" dangerouslySetInnerHTML={{ __html: contentPost.textHtml! }}></div>

                    {!isFullPage && !replyHideParent && contentPost.type == PostType.REPLY && (
                        <ParentPost post={contentPost.parentPost!} />
                    )}

                    {$authUser.value && <PostActions post={post} onUpdate={onUpdate} />}

                    {isFullPage &&
                        post.replies &&
                        post.replies.map((reply) => (
                            <PostComponent post={reply} onUpdate={onUpdate} replyHideParent key={post.id} />
                        ))}
                </div>
            </div>
        </>
    );
}

function PostOptions({ post, onUpdate }: { post: Post; onUpdate?: (post: Post | null) => void }) {
    const editPost = async (event: MouseEvent) => {
        event.stopPropagation();
        const updatedPost = await DialogService.getInstance().open<Post | null>(PostEditDialog, { post });
        if (updatedPost !== null && onUpdate) onUpdate(updatedPost);
    };

    const deletePost = async (event: MouseEvent) => {
        event.stopPropagation();
        if (
            await DialogService.getInstance().confirm(
                'Are you sure?',
                post.type === PostType.REPOST
                    ? 'Are you sure you want to delete this repost?'
                    : `Are you sure you want to delete this post, the ${post.repliesCount} replies and the ${post.repostsCount} reposts?`,
                'Delete',
                DeleteIcon,
            )
        ) {
            await PostsService.getInstance().delete(post.id);
            if (onUpdate) onUpdate(null);
        }
    };

    return (
        <div className="dropdown is-hoverable is-right is-pulled-right">
            <div className="dropdown-trigger">
                <button className="button is-small">
                    <OptionsIcon />
                </button>
            </div>
            <div className="dropdown-menu">
                <div className="dropdown-content">
                    {post.type !== PostType.REPOST && (
                        <a className="dropdown-item" href="#" onClick={editPost}>
                            <EditIcon />
                            {post.type === PostType.NORMAL ? 'Edit post' : null}
                            {post.type === PostType.REPLY ? 'Edit reply' : null}
                        </a>
                    )}
                    <a className="dropdown-item" href="#" onClick={deletePost}>
                        <DeleteIcon />
                        {post.type === PostType.NORMAL ? 'Delete post' : null}
                        {post.type === PostType.REPLY ? 'Delete reply' : null}
                        {post.type === PostType.REPOST ? 'Delete repost' : null}
                    </a>
                </div>
            </div>
        </div>
    );
}

function PostActions({ post, onUpdate }: { post: Post; onUpdate?: (post: Post) => void }) {
    const replyPost = async (event: MouseEvent) => {
        event.stopPropagation();
        const replyPost = await DialogService.getInstance().open<Post | null>(PostReplyDialog, { post });
        if (replyPost !== null) {
            route(`/posts/${replyPost.id}`);
        }
    };

    const repostPost = async (event: MouseEvent) => {
        event.stopPropagation();
        const repostedPost = await PostsService.getInstance().repost(post.id);
        if (repostedPost !== null) {
            route(`/posts/${repostedPost.id}`);
        }
    };

    const likePost = async (event: MouseEvent) => {
        event.stopPropagation();
        if (!post.authUserLiked) {
            await PostsService.getInstance().like(post.id);
            if (post.authUserDisliked) {
                post.dislikesCount--;
                post.authUserDisliked = false;
            }
            post.likesCount++;
            post.authUserLiked = true;
        } else {
            await PostsService.getInstance().remove_like(post.id);
            post.likesCount--;
            post.authUserLiked = false;
        }
        if (onUpdate) onUpdate(post);
    };

    const dislikePost = async (event: MouseEvent) => {
        event.stopPropagation();
        if (!post.authUserDisliked) {
            await PostsService.getInstance().dislike(post.id);
            if (post.authUserLiked) {
                post.likesCount--;
                post.authUserLiked = false;
            }
            post.dislikesCount++;
            post.authUserDisliked = true;
        } else {
            await PostsService.getInstance().remove_dislike(post.id);
            post.dislikesCount--;
            post.authUserDisliked = false;
        }
        if (onUpdate) onUpdate(post);
    };

    const sharePost = async (event: MouseEvent) => {
        event.stopPropagation();
        if ('share' in navigator) {
            await navigator.share({
                title: 'PlaatBook post',
                text: post.text,
                url: `${window.location.host}/posts/${post.id}`,
            });
        } else {
            await DialogService.getInstance().open(PostShareDialog, { post });
        }
    };

    return (
        <div className="buttons">
            <button className={`button is-small pl-4 py-2`} onClick={replyPost} title="Reply to post">
                <CommentIcon className="mr-2" />
                {post.repliesCount}
            </button>
            <button className={`button is-small pl-4 py-2`} onClick={repostPost} title="Repost post">
                <RepostIcon className="mr-2" />
                {post.repostsCount}
            </button>
            <button
                className={`button is-small ${post.authUserLiked ? 'is-link' : ''} pl-4 py-2`}
                onClick={likePost}
                title="Like post"
            >
                <LikeIcon className="mr-2" />
                {post.likesCount}
            </button>
            <button
                className={`button is-small ${post.authUserDisliked ? 'is-danger' : ''} pl-4 py-2`}
                onClick={dislikePost}
                title="Dislike post"
            >
                <DislikeIcon className="mr-2" />
                {post.dislikesCount}
            </button>
            <button className={`button is-small pl-4 py-2`} onClick={(event) => event.stopPropagation()}>
                <StatsIcon className="mr-2" />
                {post.viewsCount}
            </button>
            <button className={`button is-small pl-4 py-2`} onClick={sharePost}>
                <ShareIcon className="mr-2" />
                Share
            </button>
        </div>
    );
}

function ParentPost({ post }: { post: Post }) {
    const openPost = (event: MouseEvent) => {
        event.stopPropagation();
        route(`/posts/${post.id}`);
    };

    const contentPost = post.type === PostType.REPOST ? post.parentPost! : post;
    return (
        <div className="media" onClick={openPost}>
            <div className="media-left">
                <Link href={`/users/${contentPost.user!.username}`}>
                    <img className="image is-64x64" src="/images/avatar.svg" />
                </Link>
            </div>
            <div className="media-content">
                {post.type === PostType.REPLY && (
                    <div className="mb-1">
                        <Link href={`/posts/${post.parentPost!.id}`} style="color: inherit;">
                            <CommentIcon className="is-small mr-1" />
                            {post.user!.username} replied
                        </Link>
                    </div>
                )}
                {post.type === PostType.REPOST && (
                    <div className="mb-1">
                        <Link href={`/posts/${post.parentPost!.id}`} style="color: inherit;">
                            <RepostIcon className="is-small mr-1" />
                            {post.user!.username} reposted
                        </Link>
                    </div>
                )}

                <div className="mb-1" style="position: relative;">
                    <Link className="mr-2" href={`/users/${contentPost.user!.username}`}>
                        <strong>@{contentPost.user!.username}</strong>
                    </Link>
                    <small className="mr-2">{dateFormatAgo(contentPost.createdAt)}</small>
                    {contentPost.createdAt !== contentPost.updatedAt && (
                        <span className="tag" style="position: absolute; top: 0;">
                            <EditIcon className="is-small mr-1" />
                            Edited
                        </span>
                    )}
                </div>

                <div className="content" dangerouslySetInnerHTML={{ __html: contentPost.textHtml! }}></div>
            </div>
        </div>
    );
}
