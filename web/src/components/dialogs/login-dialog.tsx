/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { LoginForm } from '../forms/login-form.tsx';

export function LoginDialog({ onConfirm }: { onConfirm: (success: boolean) => void }) {
    return (
        <div className="modal is-active">
            <div className="modal-background" onClick={() => onConfirm(false)}></div>
            <LoginForm dialog onConfirm={onConfirm} />
        </div>
    );
}
