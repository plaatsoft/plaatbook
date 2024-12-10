/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useEffect, useState } from 'preact/hooks';
import { useLocation } from 'preact-iso';
import { Field } from '../components/field.tsx';
import { SearchService } from '../services/search.service.ts';
import { UserComponent } from '../components/user.tsx';
import { PostComponent } from '../components/post.tsx';
import { User } from '../models/user.ts';
import { Post } from '../models/post.ts';

export function Search() {
    const location = useLocation();
    const [query, setQuery] = useState(location.query.q || '');
    const [users, setUsers] = useState<User[]>([]);
    const [posts, setPosts] = useState<Post[]>([]);

    const search = async (event?: SubmitEvent) => {
        if (event) event.preventDefault();
        location.route(`/search?q=${query}`);
        if (query.length < 1) {
            return;
        }
        const res = await SearchService.getInstance().search(query);
        if (res !== null) {
            const { posts, users } = res;
            setUsers(users);
            setPosts(posts);
        }
    };

    useEffect(() => {
        search();
    }, []);

    return (
        <div className="section">
            <h2 className="title">Search</h2>
            <form onSubmit={search}>
                <Field
                    name="query"
                    type="text"
                    placeholder="Type to search for something..."
                    value={query}
                    setValue={setQuery}
                />
            </form>
            <hr />

            {users.length > 0 && (
                <>
                    <h3 className="subtitle">Users</h3>
                    {users.map((user) => (
                        <UserComponent key={user.id} user={user} />
                    ))}
                    {posts.length > 0 && <hr />}
                </>
            )}
            {posts.length > 0 && (
                <>
                    <h3 className="subtitle">Posts</h3>
                    {posts.map((post) => (
                        <PostComponent key={post.id} post={post} />
                    ))}
                </>
            )}
            {users.length === 0 && posts.length === 0 && <p>Type you query to search stuff!</p>}
        </div>
    );
}
