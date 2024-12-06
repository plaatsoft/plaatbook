/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useLocation } from 'preact-iso';
import { useState } from 'preact/hooks';
import { Field } from '../../components/field.tsx';
import { AuthService } from '../../services/auth.service.ts';

export function Login() {
    const authService = AuthService.getInstance();
    const location = useLocation();
    const [isLoading, setIsLoading] = useState(false);
    const [isError, setIsError] = useState(false);
    const [logon, setLogon] = useState('');
    const [password, setPassword] = useState('');

    const login = async (event: SubmitEvent) => {
        event.preventDefault();
        setIsLoading(true);
        if (await authService.login(logon, password)) {
            location.route('/');
        } else {
            setIsLoading(false);
            setIsError(true);
        }
    };

    return (
        <div className="section">
            <form className="box mx-auto" style="max-width: 50rem;" onSubmit={login}>
                <h2 className="title is-5">Login with your PlaatBook account</h2>

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
        </div>
    );
}
