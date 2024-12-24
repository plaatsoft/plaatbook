/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useState } from 'preact/hooks';
import { Post } from '../../models/post.ts';
import { Field } from '../forms/field.tsx';
import { Errors } from '../../models/errors.ts';
import { PostsService } from '../../services/posts.service.ts';
import { CommentIcon } from '../icons.tsx';
import { POST_TEXT_MAX } from '../../consts.ts';

export function PostReplyDialog({ post, onConfirm }: { post: Post; onConfirm: (updatedPost: Post | null) => void }) {
    const [isLoading, setIsLoading] = useState(false);
    const [text, setText] = useState('');
    const [errors, setErrors] = useState<Errors>({});

    const update = async (event: SubmitEvent) => {
        event.preventDefault();
        setIsLoading(true);
        setErrors({});
        const [success, result] = await PostsService.getInstance().reply(post.id, text);
        if (success) {
            onConfirm(result as Post);
        } else {
            setIsLoading(false);
            setErrors(result as Errors);
        }
    };

    return (
        <div className="modal is-active">
            <div className="modal-background" onClick={() => onConfirm(null)}></div>
            <form className="modal-card" onSubmit={update}>
                <header className="modal-card-head">
                    <p className="modal-card-title">Reply to post</p>
                    <button type="button" className="delete" onClick={() => onConfirm(null)}></button>
                </header>
                <section className="modal-card-body">
                    <Field
                        type="textarea"
                        placeholder="What's your reply?"
                        value={text}
                        setValue={setText}
                        help={
                            <span className="is-pulled-right has-text-weight-bold has-text-grey">
                                {POST_TEXT_MAX - text.length}
                            </span>
                        }
                        error={errors.text?.join(', ')}
                        disabled={isLoading}
                        autofocus
                    />
                </section>
                <footer className="modal-card-foot">
                    <div className="buttons">
                        <button type="submit" className="button is-link">
                            <CommentIcon className="mr-2" />
                            Reply
                        </button>
                        <button type="button" className="button" onClick={() => onConfirm(null)}>
                            Cancel
                        </button>
                    </div>
                </footer>
            </form>
        </div>
    );
}
