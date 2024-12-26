/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useEffect, useState } from 'preact/hooks';
import { UsersService } from '../../services/users.service.ts';
import { User, Post } from '../../api.ts';
import { NotFound } from '../not-found.tsx';
import { PostComponent } from '../../components/post.tsx';
import { dateFormatAgo } from '../../utils.ts';
import { $authUser } from '../../services/auth.service.ts';
import { $addPost } from '../../services/posts.service.ts';
import { PostCreateForm } from '../../components/forms/post-create-form.tsx';
import { InfiniteList } from '../../components/infinite-list.tsx';
import { BirthdateIcon, CalendarIcon, EditIcon, LinkIcon, LocationIcon, OptionsIcon } from '../../components/icons.tsx';
import { DialogService } from '../../services/dialog.service.tsx';
import { UserEditDialog } from '../../components/dialogs/user-edit-dialog.tsx';
import { Link } from '../../router.tsx';

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

    const editProfile = async (event: MouseEvent) => {
        event.preventDefault();
        const updatedUser = await DialogService.getInstance().open<User>(UserEditDialog, { user: user! });
        console.log('close', updatedUser);
        if (updatedUser) setUser(updatedUser);
    };

    return (
        <>
            {user !== null && user !== undefined && (
                <div className="section">
                    <div className="media">
                        <div className="media-left">
                            <img className="image is-64x64" src="/images/avatar.svg" />
                        </div>
                        <div className="media-content">
                            {user.id === $authUser.value?.id && (
                                <div className="dropdown is-hoverable is-right is-pulled-right">
                                    <div className="dropdown-trigger">
                                        <button className="button is-small">
                                            <OptionsIcon />
                                        </button>
                                    </div>
                                    <div className="dropdown-menu">
                                        <div className="dropdown-content">
                                            <a className="dropdown-item" href="#" onClick={editProfile}>
                                                <EditIcon />
                                                Edit profile
                                            </a>
                                        </div>
                                    </div>
                                </div>
                            )}

                            <h2 className="title mb-2">
                                <Link href={`/users/${user.username}`} style="color: inherit;">
                                    {user.firstname && `${user.firstname} `}
                                    {user.lastname && `${user.lastname} `}@{user.username}
                                </Link>
                            </h2>
                            {user.bio && <p className="mb-3">{user.bio}</p>}
                            <p>
                                {user.location && (
                                    <span className="tag mr-2" title="User location">
                                        <LocationIcon className="is-small mr-1" />
                                        {user.location}
                                    </span>
                                )}
                                {user.website && (
                                    <span className="tag mr-2" title="User website">
                                        <LinkIcon className="is-small mr-1" />
                                        <a href={user.website} target="_blank" rel="noreferrer" style="color: inherit;">
                                            {user.website}
                                        </a>
                                    </span>
                                )}
                                {user.birthdate && (
                                    <span className="tag mr-2" title="User birthdate">
                                        <BirthdateIcon className="is-small mr-1" />
                                        {user.birthdate}
                                    </span>
                                )}
                                <span className="tag" title="User joined date">
                                    <CalendarIcon className="is-small mr-1" />
                                    Joined {dateFormatAgo(user.created_at)}
                                </span>
                            </p>
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
