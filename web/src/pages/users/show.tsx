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

export function UsersShow({ user_id }: { user_id: string }) {
    const [user, setUser] = useState<User | null>(null);

    const getUser = async () => {
        const user = await UsersService.getInstance().get(user_id);
        setUser(user);
    };

    useEffect(() => {
        getUser();
    }, [user_id]);

    return user !== undefined && user !== null ? (
        <div className="section">
            <div className="media has-background-dark px-5" style="height: 10rem; align-items: center;">
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

            <UserPostsList user={user} />
        </div>
    ) : (
        <NotFound />
    );
}

function UserPostsList({ user }: { user: User }) {
    const [posts, setPosts] = useState<Post[]>([]);

    const fetchPosts = async () => {
        const posts = await UsersService.getInstance().getPosts(user.id);
        if (posts === null) {
            return;
        }
        setPosts(
            posts.map((post) => {
                post.user = user;
                return post;
            }),
        );
    };
    useEffect(() => {
        fetchPosts();
    }, []);

    return (
        <>
            {posts.map((post) => (
                <PostComponent post={post} key={post.id} />
            ))}
        </>
    );
}
