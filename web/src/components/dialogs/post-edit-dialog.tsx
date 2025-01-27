/*
 * Copyright (c) 2024-2025 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useState } from 'preact/hooks';
import { Post, PostType, Report } from '../../api.ts';
import { Field } from '../forms/field.tsx';
import { PostsService } from '../../services/posts.service.ts';
import { EditIcon } from '../icons.tsx';
import { POST_TEXT_MAX } from '../../consts.ts';

export function PostEditDialog({ post, onConfirm }: { post: Post; onConfirm: (updatedPost: Post | null) => void }) {
    const [isLoading, setIsLoading] = useState(false);
    const [text, setText] = useState(post.text!);
    const [report, setReport] = useState<Report>({});

    const update = async (event: SubmitEvent) => {
        event.preventDefault();
        setIsLoading(true);
        setReport({});
        const [success, result] = await PostsService.getInstance().update(post.id, text);
        if (success) {
            onConfirm(result as Post);
        } else {
            setIsLoading(false);
            setReport(result as Report);
        }
    };

    return (
        <div className="modal is-active">
            <div className="modal-background" onClick={() => onConfirm(null)}></div>
            <form className="modal-card" onSubmit={update}>
                <header className="modal-card-head">
                    <p className="modal-card-title">
                        {post.type == PostType.NORMAL && <>Edit post</>}
                        {post.type == PostType.REPLY && <>Edit reply</>}
                    </p>
                    <button type="button" className="delete" onClick={() => onConfirm(null)}></button>
                </header>
                <section className="modal-card-body">
                    <Field
                        type="textarea"
                        placeholder="Post text"
                        value={text}
                        setValue={setText}
                        help={
                            <span className="is-pulled-right has-text-weight-bold has-text-grey">
                                {POST_TEXT_MAX - text.length}
                            </span>
                        }
                        error={report.text?.join(', ')}
                        disabled={isLoading}
                        autofocus
                    />
                </section>
                <footer className="modal-card-foot">
                    <div className="buttons">
                        <button type="submit" className="button is-link">
                            <EditIcon className="mr-2" />
                            Edit
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
