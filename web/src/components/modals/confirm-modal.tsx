/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

export function ConfirmModal({
    title,
    message,
    action,
    onConfirm,
}: {
    title: string;
    message: string;
    action: string;
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
                            className={`button ${action === 'Delete' ? 'is-danger' : 'is-success'}`}
                            onClick={() => onConfirm(true)}
                        >
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
