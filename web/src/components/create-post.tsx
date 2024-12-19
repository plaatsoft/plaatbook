/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useState } from 'preact/hooks';
import { PostsService, $refreshPosts } from '../services/posts.service.ts';
import { Errors } from '../models/errors.ts';
import { Field } from '../components/field.tsx';

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
                            <svg className="icon mr-2" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                                <path d="M12,3C17.5,3 22,6.58 22,11C22,15.42 17.5,19 12,19C10.76,19 9.57,18.82 8.47,18.5C5.55,21 2,21 2,21C4.33,18.67 4.7,17.1 4.75,16.5C3.05,15.07 2,13.13 2,11C2,6.58 6.5,3 12,3Z" />
                            </svg>
                            Post
                        </button>
                    </div>
                </div>
            </form>
            <hr />
        </>
    );
}
