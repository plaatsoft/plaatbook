/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useState } from 'preact/hooks';
import { PostsService, $refreshPosts } from '../services/posts.service.ts';
import { Errors } from '../models/errors.ts';
import { Field } from '../components/field.tsx';
import { CommentIcon } from './icons.tsx';

export function CreatePost() {
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
            $refreshPosts.value = $refreshPosts.value + 1;
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
                    <Field
                        name="text"
                        type="textarea"
                        placeholder="What's on your mind?"
                        value={text}
                        setValue={setText}
                        error={errors.text?.join(', ')}
                        disabled={isLoading}
                    />

                    <div className="field">
                        <button className="button is-link" type="submit">
                            <CommentIcon className="mr-2" />
                            Post
                        </button>
                    </div>
                </div>
            </form>
            <hr />
        </>
    );
}
