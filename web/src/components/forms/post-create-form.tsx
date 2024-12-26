/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useState } from 'preact/hooks';
import { PostsService, $addPost } from '../../services/posts.service.ts';
import { Report, Post } from '../../api.ts';
import { Field } from './field.tsx';
import { CommentIcon } from '../icons.tsx';
import { POST_TEXT_MAX } from '../../consts.ts';
import { $authUser } from '../../services/auth.service.ts';
import { Link } from '../../router.tsx';

export function PostCreateForm() {
    const [isLoading, setIsLoading] = useState(false);
    const [text, setText] = useState('');
    const [report, setReport] = useState<Report>({});

    const createPost = async (event: SubmitEvent) => {
        event.preventDefault();
        setIsLoading(true);
        setReport({});
        const [success, result] = await PostsService.getInstance().create(text);
        setIsLoading(false);
        if (success) {
            setText('');
            $addPost.value = result as Post;
        } else {
            setReport(result as Report);
        }
    };

    return (
        <>
            <form className="media" onSubmit={createPost}>
                <div className="media-left">
                    <Link href={`/users/${$authUser.value!.username}`}>
                        <img className="image is-64x64" src="/images/avatar.svg" />{' '}
                    </Link>
                </div>
                <div className="media-content">
                    <Field
                        type="textarea"
                        placeholder="What's on your mind?"
                        value={text}
                        setValue={setText}
                        error={report.text?.join(', ')}
                        disabled={isLoading}
                    />

                    <div className="field">
                        <button type="submit" className="button is-link">
                            <CommentIcon className="mr-2" />
                            Post
                        </button>
                        <span className="is-pulled-right has-text-weight-bold has-text-grey">
                            {POST_TEXT_MAX - text.length}
                        </span>
                    </div>
                </div>
            </form>
            <hr />
        </>
    );
}
