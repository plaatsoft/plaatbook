/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useState } from 'preact/hooks';
import { $authUser, AuthService } from '../services/auth.service.ts';
import { Errors } from '../models/errors.ts';
import { Field } from '../components/field.tsx';
import { Notification } from '../components/notification.tsx';

export function Settings() {
    return (
        <div className="section">
            <h2 className="title">Settings</h2>
            <ChangeDetailsForm />
            <ChangePasswordForm />
        </div>
    );
}

function ChangeDetailsForm() {
    const authService = AuthService.getInstance();
    const [isLoading, setIsLoading] = useState(false);
    const [isDone, setIsDone] = useState(false);
    const [username, setUsername] = useState($authUser.value?.username ?? '');
    const [email, setEmail] = useState($authUser.value?.email ?? '');
    const [errors, setErrors] = useState<Errors>({});

    const changeDetails = async (event: SubmitEvent) => {
        event.preventDefault();
        setIsLoading(true);
        setErrors({});
        const errors = await authService.changeDetails(username, email);
        setIsLoading(false);
        if (errors === null) {
            setIsDone(true);
        } else {
            setErrors(errors);
        }
    };

    return (
        <form className="box" onSubmit={changeDetails}>
            <h2 className="title is-5">Change account details</h2>

            {isDone && <Notification text="Account details saved" />}

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

            <div className="field">
                <button className="button is-link" type="submit">
                    Change
                </button>
            </div>
        </form>
    );
}

function ChangePasswordForm() {
    const authService = AuthService.getInstance();
    const [isLoading, setIsLoading] = useState(false);
    const [isDone, setIsDone] = useState(false);
    const [currentPassword, setCurrentPassword] = useState('');
    const [password, setPassword] = useState('');
    const [confirmPassword, setConfirmPassword] = useState('');
    const [errors, setErrors] = useState<Errors>({});

    const changePassword = async (event: SubmitEvent) => {
        event.preventDefault();
        if (password !== confirmPassword) {
            setErrors({ confirm_password: ['Passwords do not match'] });
            return;
        }
        setIsLoading(true);
        setErrors({});
        const errors = await authService.changePassword(currentPassword, password);
        setIsLoading(false);
        if (errors === null) {
            setIsDone(true);
        } else {
            if (password !== confirmPassword) {
                errors.confirm_password = ['Passwords do not match'];
            }
            setErrors(errors);
        }
    };

    return (
        <form className="box" onSubmit={changePassword}>
            <h2 className="title is-5">Change account password</h2>

            {isDone && <Notification text="New password saved" />}

            <Field
                name="current_password"
                type="password"
                label="Current password"
                value={currentPassword}
                setValue={setCurrentPassword}
                error={errors.current_password?.join(', ')}
                disabled={isLoading}
            />

            <Field
                name="password"
                type="password"
                label="New password"
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
                    Change password
                </button>
            </div>
        </form>
    );
}
