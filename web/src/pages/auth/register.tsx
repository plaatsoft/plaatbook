/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useLocation } from 'preact-iso';
import { useState } from 'preact/hooks';
import { Field } from '../../components/field.tsx';
import { AuthService } from '../../services/auth.service.ts';
import { Errors } from '../../models/index.ts';

export function Register() {
    const authService = AuthService.getInstance();
    const location = useLocation();
    const [isLoading, setIsLoading] = useState(false);
    const [username, setUsername] = useState('');
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [errors, setErrors] = useState<Errors>({});

    const register = async (event: SubmitEvent) => {
        event.preventDefault();
        setIsLoading(true);
        const errors = await authService.register(username, email, password);
        if (errors === undefined) {
            await authService.login(email, password);
            location.route('/');
        } else {
            setIsLoading(false);
            setErrors(errors);
        }
    };

    return (
        <div className="section">
            <form className="box mx-auto" style="max-width: 50rem;" onSubmit={register}>
                <h2 className="title is-5">Register a new PlaatBook account</h2>

                <Field
                    name="username"
                    type="text"
                    label="Username"
                    value={username}
                    setValue={setUsername}
                    error={errors.username?.join(', ')}
                    disabled={isLoading}
                    autofocus
                />

                <Field
                    name="email"
                    type="email"
                    label="Email address"
                    value={email}
                    setValue={setEmail}
                    error={errors.email?.join(', ')}
                    disabled={isLoading}
                />

                <Field
                    name="password"
                    type="password"
                    label="Password"
                    value={password}
                    setValue={setPassword}
                    error={errors.password?.join(', ')}
                    disabled={isLoading}
                />

                <div className="field">
                    <button className="button is-link" type="submit">
                        Register
                    </button>
                </div>
            </form>
        </div>
    );
}
