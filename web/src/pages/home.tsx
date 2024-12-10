/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useState, useEffect } from 'preact/hooks';
import { $authUser } from '../services/auth.service.ts';
import { PostsService } from '../services/posts.service.ts';
import { Errors } from '../models/errors.ts';
import { Post } from '../models/post.ts';
import { PostComponent } from '../components/post.tsx';
import { signal } from '@preact/signals';

export function Home() {
    return (
        <div className="section">
            {$authUser.value !== null ? (
                <h2 className="title">Welcome {$authUser.value?.username}!</h2>
            ) : (
                <h2 className="title">Home</h2>
            )}

            {$authUser.value !== null ? <CreatePostForm /> : null}

            <PostsList />
        </div>
    );
}

const refreshPosts = signal<number>(0);

function CreatePostForm() {
    const [isLoading, setIsLoading] = useState(false);
    const [text, setText] = useState('');
    const [errors, setErrors] = useState<Errors>({});

    const createPost = async (event: SubmitEvent) => {
        event.preventDefault();
        setIsLoading(true);
        setErrors({});
        const errors = await PostsService.getInstance().create(text);
        setIsLoading(false);
        if (errors === null) {
            setText('');
            refreshPosts.value = Date.now();
        } else {
            setErrors(errors);
        }
    };

    return (
        <>
            <form className="media" onSubmit={createPost}>
                <div className="media-left">
                    <img className="image is-64x64" src="/images/avatar.svg" />
                </div>
                <div className="media-content">
                    <div className="field">
                        <textarea
                            className={`textarea ${errors.text !== undefined ? 'is-danger' : ''}`}
                            placeholder="What's on your mind?"
                            rows={3}
                            value={text}
                            onInput={(event) => setText((event.target as HTMLTextAreaElement).value)}
                            disabled={isLoading}
                        />
                        {errors.text !== undefined && <p className="help is-danger">{errors.text.join(', ')}</p>}
                    </div>

                    <div className="field">
                        <button className="button is-link" type="submit">
                            Post
                        </button>
                    </div>
                </div>
            </form>
            <hr />
        </>
    );
}

function PostsList() {
    const [posts, setPosts] = useState<Post[]>([]);

    const fetchPosts = async () => {
        setPosts(await PostsService.getInstance().getAll());
    };
    useEffect(() => {
        fetchPosts();
    }, [refreshPosts.value]);

    return (
        <>
            {posts.map((post) => (
                <PostComponent post={post} key={post.id} />
            ))}
        </>
    );
}
