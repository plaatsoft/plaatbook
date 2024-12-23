/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useEffect, useState } from 'preact/hooks';
import { UsersService } from '../../services/users.service.ts';
import { User } from '../../models/user.ts';
import { NotFound } from '../not-found.tsx';
import { Post } from '../../models/post.ts';
import { PostComponent } from '../../components/post.tsx';
import { dateFormatAgo } from '../../utils.ts';
import { $authUser } from '../../services/auth.service.ts';
import { $addPost } from '../../services/posts.service.ts';
import { PostCreateForm } from '../../components/post-create-form.tsx';
import { InfiniteList } from '../../components/infinite-list.tsx';

const styles = css`
    .user-hero {
        height: 8rem;
        align-items: center !important;
    }
`;

export function UsersShow({ user_id }: { user_id: string }) {
    const [user, setUser] = useState<User | null | undefined>(undefined);

    const getUser = async () => {
        const user = await UsersService.getInstance().get(user_id);
        setUser(user);
    };
    useEffect(() => {
        getUser();
    }, [user_id]);

    useEffect(() => {
        if (user) document.title = `${user!.username} - PlaatBook`;
    }, [user]);

    return (
        <>
            {user !== null && user !== undefined && (
                <div className="section">
                    <div className={`media ${styles['user-hero']}`}>
                        <div className="media-left">
                            <img className="image is-64x64" src="/images/avatar.svg" />
                        </div>
                        <div className="media-content">
                            <h2 className="title mb-2">
                                <a href={`/users/${user.username}`}>@{user.username}</a>
                            </h2>
                            <p>Joined {dateFormatAgo(user.created_at)}</p>
                        </div>
                    </div>

                    {user.id === $authUser.value?.id && <PostCreateForm />}

                    <UserPostsList user={user} />
                </div>
            )}
            {user === null && <NotFound />}
        </>
    );
}

function UserPostsList({ user }: { user: User }) {
    const [posts, setPosts] = useState<Post[]>([]);

    useEffect(() => {
        if ($addPost.value) posts.unshift($addPost.value);
        $addPost.value = null;
    }, [$addPost.value]);

    return (
        <InfiniteList
            items={posts}
            fetchPage={async (page) => {
                const newPosts = (await UsersService.getInstance().getPosts(user.id, page))!;
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
