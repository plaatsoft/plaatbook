/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

// eslint-disable-next-line import/named
import { FunctionComponent } from 'preact';

export function ConfirmModal({
    title,
    message,
    action,
    ActionIcon,
    onConfirm,
}: {
    title: string;
    message: string;
    action: string;
    ActionIcon?: FunctionComponent<{ className: string }>;
    onConfirm: (confirmed: boolean) => void;
}) {
    return (
        <div className="modal is-active">
            <div className="modal-background" onClick={() => onConfirm(false)}></div>
            <div className="modal-card">
                <header className="modal-card-head">
                    <p className="modal-card-title">{title}</p>
                    <button className="delete" aria-label="close" onClick={() => onConfirm(false)}></button>
                </header>
                <section className="modal-card-body">
                    <p>{message}</p>
                </section>
                <footer className="modal-card-foot">
                    <div className="buttons">
                        <button
                            className={`button ${action === 'Delete' || action === 'Revoke' ? 'is-danger' : 'is-success'}`}
                            onClick={() => onConfirm(true)}
                        >
                            {ActionIcon && <ActionIcon className="mr-2" />}
                            {action}
                        </button>
                        <button className="button" onClick={() => onConfirm(false)}>
                            Cancel
                        </button>
                    </div>
                </footer>
            </div>
        </div>
    );
}
