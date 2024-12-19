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

const styles = css`
    .no-results {
        color: var(--bulma-grey-light);
        font-style: italic;
    }
`;

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
                <div className="field has-addons">
                    <Field
                        name="query"
                        type="text"
                        placeholder="Type to search for something..."
                        value={query}
                        setValue={setQuery}
                        addon
                        expanded
                        autofocus
                    />
                    <div className="control">
                        <button type="submit" className="button is-link">
                            <svg className="icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                                <path d="M9.5,3A6.5,6.5 0 0,1 16,9.5C16,11.11 15.41,12.59 14.44,13.73L14.71,14H15.5L20.5,19L19,20.5L14,15.5V14.71L13.73,14.44C12.59,15.41 11.11,16 9.5,16A6.5,6.5 0 0,1 3,9.5A6.5,6.5 0 0,1 9.5,3M9.5,5C7,5 5,7 5,9.5C5,12 7,14 9.5,14C12,14 14,12 14,9.5C14,7 12,5 9.5,5Z" />
                            </svg>
                        </button>
                    </div>
                </div>
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
                            onUpdate={(post) => setPosts(posts.map((p) => (p.id === post.id ? post : p)))}
                            key={post.id}
                        />
                    ))}
                </>
            )}
            {users.length === 0 && posts.length === 0 && <p className={styles['no-results']}>Nothing searched yet</p>}
        </div>
    );
}
