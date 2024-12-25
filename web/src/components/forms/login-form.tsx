/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useState } from 'preact/hooks';
import { Field } from './field.tsx';
import { AuthService } from '../../services/auth.service.ts';
import { LoginIcon } from '../icons.tsx';
import { route } from '../../router.tsx';

export function LoginForm({ dialog, onConfirm }: { dialog?: boolean; onConfirm?: (success: boolean) => void }) {
    const [isLoading, setIsLoading] = useState(false);
    const [isError, setIsError] = useState(false);
    const [logon, setLogon] = useState('');
    const [password, setPassword] = useState('');

    const login = async (event: SubmitEvent) => {
        event.preventDefault();
        setIsLoading(true);
        setIsError(false);
        if (await AuthService.getInstance().login(logon, password)) {
            if (dialog) {
                onConfirm!(true);
            } else {
                route('/');
            }
        } else {
            setIsLoading(false);
            setIsError(true);
        }
    };

    const fields = (
        <>
            <Field
                type="text"
                label="Username or email address"
                value={logon}
                setValue={setLogon}
                error={isError ? 'Invalid username, email address or password' : undefined}
                disabled={isLoading}
                autofocus
            />

            <Field
                type="password"
                label="Password"
                value={password}
                setValue={setPassword}
                error={isError ? '' : undefined}
                disabled={isLoading}
            />
        </>
    );

    return dialog ? (
        <form className="modal-card" onSubmit={login}>
            <header className="modal-card-head">
                <p className="modal-card-title">Add account</p>
                <button type="button" className="delete" onClick={() => onConfirm!(false)}></button>
            </header>
            <section className="modal-card-body">{fields}</section>
            <footer className="modal-card-foot">
                <div className="buttons">
                    <button type="submit" className="button is-link">
                        <LoginIcon className="mr-2" />
                        Login
                    </button>
                    <button type="button" className="button" onClick={() => onConfirm!(false)}>
                        Cancel
                    </button>
                </div>
            </footer>
        </form>
    ) : (
        <form className="section" onSubmit={login}>
            <h2 className="title">Login account</h2>
            {fields}
            <div className="field">
                <button type="submit" className="button is-link">
                    <LoginIcon className="mr-2" />
                    Login
                </button>
            </div>
        </form>
    );
}
