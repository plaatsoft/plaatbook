/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useLocation } from 'preact-iso';
import { useState } from 'preact/hooks';
import { Field } from '../../components/field.tsx';
import { AuthService } from '../../services/auth.service.ts';
import { Errors } from '../../models/errors.ts';

export function Register() {
    const location = useLocation();
    const [isLoading, setIsLoading] = useState(false);
    const [username, setUsername] = useState('');
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [confirmPassword, setConfirmPassword] = useState('');
    const [errors, setErrors] = useState<Errors>({});

    const register = async (event: SubmitEvent) => {
        event.preventDefault();
        if (password !== confirmPassword) {
            setErrors({ confirm_password: ['Passwords do not match'] });
            return;
        }
        setIsLoading(true);
        setErrors({});
        const errors = await AuthService.getInstance().register(username, email, password);
        if (errors === null) {
            await AuthService.getInstance().login(email, password);
            location.route('/');
        } else {
            if (password !== confirmPassword) {
                errors.confirm_password = ['Passwords do not match'];
            }
            setIsLoading(false);
            setErrors(errors);
        }
    };

    return (
        <form className="section" onSubmit={register}>
            <h2 className="title">Register</h2>

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

            <Field
                name="confirm_password"
                type="password"
                label="Confirm new password"
                value={confirmPassword}
                setValue={setConfirmPassword}
                disabled={isLoading}
                error={errors.confirm_password?.join(', ')}
            />

            <div className="field">
                <button className="button is-link" type="submit">
                    Register
                </button>
            </div>
        </form>
    );
}
