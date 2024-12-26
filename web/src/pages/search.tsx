/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useEffect, useState } from 'preact/hooks';
import { Field } from '../components/forms/field.tsx';
import { SearchService } from '../services/search.service.ts';
import { UserComponent } from '../components/user.tsx';
import { PostComponent } from '../components/post.tsx';
import { User, Post } from '../api.ts';
import { SearchIcon } from '../components/icons.tsx';
import { route } from '../router.tsx';

const styles = css`
    .no-results {
        color: var(--bulma-grey-light);
        font-style: italic;
    }
`;

export function Search() {
    const queryParams = new URLSearchParams(window.location.search);
    const [query, setQuery] = useState(queryParams.get('q') || '');
    const [users, setUsers] = useState<User[]>([]);
    const [posts, setPosts] = useState<Post[]>([]);

    const updateTitle = () => {
        document.title = `${query ? `${query} - ` : ''}Search - PlaatBook`;
    };
    useEffect(() => updateTitle, []);

    const search = async (event?: SubmitEvent) => {
        if (event) event.preventDefault();
        if (query.length < 1) {
            return;
        }
        route(`/search?q=${query}`);
        updateTitle();
        const res = await SearchService.getInstance().search(query, 1);
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
                    type="text"
                    placeholder="Type to search for something..."
                    value={query}
                    setValue={setQuery}
                    addonPost={
                        <button type="submit" className="button is-link">
                            <SearchIcon />
                        </button>
                    }
                    autofocus
                />
            </form>
            <hr />

            {users.length > 0 && (
                <>
                    <h3 className="subtitle">Users</h3>
                    {users.map((user) => (
                        <UserComponent user={user} key={user.id} />
                    ))}
                    {posts.length > 0 && <hr />}
                </>
            )}
            {posts.length > 0 && (
                <>
                    <h3 className="subtitle">Posts</h3>
                    {posts.map((post) => (
                        <PostComponent
                            post={post}
                            onUpdate={(updatedPost) => {
                                if (updatedPost)
                                    setPosts((posts) => posts.map((p) => (p.id === post.id ? updatedPost : p)));
                                else setPosts((posts) => posts.filter((p) => p.id !== post.id));
                            }}
                            key={post.id}
                        />
                    ))}
                </>
            )}
            {users.length === 0 && posts.length === 0 && <p className={styles['no-results']}>Nothing searched yet</p>}
        </div>
    );
}
