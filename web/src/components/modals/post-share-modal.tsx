/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { Post } from '../../models/post.ts';
import { CopyIcon } from '../icons.tsx';

export function PostShareModal({ post, onConfirm }: { post: Post; onConfirm: (updatedPost: null) => void }) {
    const copyUrl = async () => {
        await navigator.clipboard.writeText(`${window.location.host}/posts/${post.id}`);
    };

    return (
        <div className="modal is-active">
            <div className="modal-background" onClick={() => onConfirm(null)}></div>
            <div className="modal-card">
                <header className="modal-card-head">
                    <p className="modal-card-title">Share post</p>
                    <button type="button" className="delete" onClick={() => onConfirm(null)}></button>
                </header>
                <footer className="modal-card-foot">
                    <div className="buttons">
                        <button type="button" className="button is-link" onClick={copyUrl}>
                            <CopyIcon className="mr-2" />
                            Copy url
                        </button>
                        <button type="button" className="button" onClick={() => onConfirm(null)}>
                            Cancel
                        </button>
                    </div>
                </footer>
            </div>
        </div>
    );
}
