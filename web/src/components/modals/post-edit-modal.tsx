/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useState } from 'preact/hooks';
import { Post } from '../../models/post.ts';
import { Field } from '../field.tsx';
import { Errors } from '../../models/errors.ts';
import { PostsService } from '../../services/posts.service.ts';

export function PostEditModal({ post, onConfirm }: { post: Post; onConfirm: (updatedPost: Post | null) => void }) {
    const [isLoading, setIsLoading] = useState(false);
    const [text, setText] = useState(post.text);
    const [errors, setErrors] = useState<Errors>({});

    const update = async (event: SubmitEvent) => {
        event.preventDefault();
        setIsLoading(true);
        setErrors({});
        const errors = await PostsService.getInstance().update(post.id, text);
        if (errors === null) {
            onConfirm({ ...post, text });
        } else {
            setIsLoading(false);
            setErrors(errors);
        }
    };

    return (
        <div className="modal is-active">
            <div className="modal-background" onClick={() => onConfirm(null)}></div>
            <form className="modal-card" onSubmit={update}>
                <header className="modal-card-head">
                    <p className="modal-card-title">Edit post</p>
                    <button className="delete" aria-label="close" onClick={() => onConfirm(null)}></button>
                </header>
                <section className="modal-card-body">
                    <Field
                        name="text"
                        type="textarea"
                        placeholder="Post text"
                        value={text}
                        setValue={setText}
                        error={errors.text?.join(', ')}
                        disabled={isLoading}
                    />
                </section>
                <footer className="modal-card-foot">
                    <div className="buttons">
                        <button type="submit" className="button is-link">
                            Update
                        </button>
                        <button className="button" onClick={() => onConfirm(null)}>
                            Cancel
                        </button>
                    </div>
                </footer>
            </form>
        </div>
    );
}
