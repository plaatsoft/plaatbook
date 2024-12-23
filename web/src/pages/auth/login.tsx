/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useEffect, useState } from 'preact/hooks';
import { route } from 'preact-router';
import { Field } from '../../components/field.tsx';
import { AuthService } from '../../services/auth.service.ts';

export function Login() {
    const [isLoading, setIsLoading] = useState(false);
    const [isError, setIsError] = useState(false);
    const [logon, setLogon] = useState('');
    const [password, setPassword] = useState('');

    useEffect(() => {
        document.title = 'Login - PlaatBook';
    }, []);

    const login = async (event: SubmitEvent) => {
        event.preventDefault();
        setIsLoading(true);
        setIsError(false);
        if (await AuthService.getInstance().login(logon, password)) {
            route('/');
        } else {
            setIsLoading(false);
            setIsError(true);
        }
    };

    return (
        <form className="section" onSubmit={login}>
            <h2 className="title">Login</h2>

            <Field
                name="logon"
                type="text"
                label="Username or email address"
                value={logon}
                setValue={setLogon}
                error={isError ? 'Invalid username, email address or password' : undefined}
                disabled={isLoading}
                autofocus
            />

            <Field
                name="password"
                type="password"
                label="Password"
                value={password}
                setValue={setPassword}
                error={isError ? '' : undefined}
                disabled={isLoading}
            />

            <div className="field">
                <button className="button is-link" type="submit">
                    Login
                </button>
            </div>
        </form>
    );
}
