/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useEffect, useState } from 'preact/hooks';
import { Field } from '../../components/forms/field.tsx';
import { AuthService } from '../../services/auth.service.ts';
import { Errors } from '../../models/errors.ts';
import { RegisterIcon } from '../../components/icons.tsx';
import { route } from '../../router.tsx';

export function Register() {
    const [isLoading, setIsLoading] = useState(false);
    const [username, setUsername] = useState('');
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [confirmPassword, setConfirmPassword] = useState('');
    const [errors, setErrors] = useState<Errors>({});

    useEffect(() => {
        document.title = 'Register - PlaatBook';
    }, []);

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
            route('/');
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
            <h2 className="title">Register account</h2>

            <Field
                type="text"
                label="Username"
                value={username}
                setValue={setUsername}
                error={errors.username?.join(', ')}
                disabled={isLoading}
                autofocus
            />

            <Field
                type="email"
                label="Email address"
                value={email}
                setValue={setEmail}
                error={errors.email?.join(', ')}
                disabled={isLoading}
            />

            <Field
                type="password"
                label="Password"
                value={password}
                setValue={setPassword}
                error={errors.password?.join(', ')}
                disabled={isLoading}
            />

            <Field
                type="password"
                label="Confirm new password"
                value={confirmPassword}
                setValue={setConfirmPassword}
                disabled={isLoading}
                error={errors.confirm_password?.join(', ')}
            />

            <div className="field">
                <button type="submit" className="button is-link">
                    <RegisterIcon className="mr-2" />
                    Register
                </button>
            </div>
        </form>
    );
}
