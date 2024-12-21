/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useState, useEffect } from 'preact/hooks';
import { $authSession, $authUser, AuthService } from '../services/auth.service.ts';
import { Errors } from '../models/errors.ts';
import { Field } from '../components/field.tsx';
import { Notification } from '../components/notification.tsx';
import { Session } from '../models/session.ts';
import { dateFormat } from '../utils.ts';
import { useLocation } from 'preact-iso';
import { DialogService } from '../services/dialog.service.tsx';
import { AccountEditIcon, DeleteIcon, KeyIcon, OptionsIcon, SecurityEditIcon } from '../components/icons.tsx';

export function Settings() {
    return (
        <div className="section">
            <h2 className="title">Settings</h2>
            <ChangeDetailsForm />
            <ChangePasswordForm />
            <SessionsManagement />
        </div>
    );
}

function ChangeDetailsForm() {
    const [isLoading, setIsLoading] = useState(false);
    const [isDone, setIsDone] = useState(false);
    const [username, setUsername] = useState($authUser.value?.username ?? '');
    const [email, setEmail] = useState($authUser.value?.email ?? '');
    const [errors, setErrors] = useState<Errors>({});

    const changeDetails = async (event: SubmitEvent) => {
        event.preventDefault();
        setIsLoading(true);
        setErrors({});
        const errors = await AuthService.getInstance().changeDetails(username, email);
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
                    <AccountEditIcon className="mr-2" />
                    Change details
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
                    <SecurityEditIcon className="mr-2" />
                    Change password
                </button>
            </div>
        </form>
    );
}

function SessionsManagement() {
    const location = useLocation();
    const [sessions, setSessions] = useState<Session[]>([]);

    const fetchSessions = async () => {
        setSessions(await AuthService.getInstance().getActiveSessions());
    };
    useEffect(() => {
        fetchSessions();
    }, []);

    const revokeSession = async (session: Session) => {
        if (
            await DialogService.getInstance().confirm(
                'Are you sure?',
                'Are you sure you want to revoke this session?',
                'Revoke',
                DeleteIcon,
            )
        ) {
            if (await AuthService.getInstance().revokeSession(location, session)) {
                setSessions(sessions.filter((s) => s.id !== session.id));
            }
        }
    };

    return (
        <div className="box">
            <h2 className="title is-5">Active sessions</h2>
            {sessions.map((session) => (
                <div className="media" key={session.id}>
                    <div className="media-content">
                        <div className="dropdown is-hoverable is-right is-pulled-right">
                            <div className="dropdown-trigger">
                                <button className="button is-small">
                                    <OptionsIcon />
                                </button>
                            </div>
                            <div className="dropdown-menu">
                                <div className="dropdown-content">
                                    <a className="dropdown-item" href="#" onClick={() => revokeSession(session)}>
                                        <DeleteIcon />
                                        Revoke session
                                    </a>
                                </div>
                            </div>
                        </div>

                        <h3 className="subtitle mb-2">
                            {session.client_name} on {session.client_os}
                            {session.id === $authSession.value!.id && (
                                <span className="tag ml-2">
                                    <KeyIcon className="is-small mr-1" />
                                    Current
                                </span>
                            )}
                        </h3>
                        <p>
                            <strong>Location</strong> with {session.ip_address} at {session.ip_city},{' '}
                            {session.ip_country}
                        </p>
                        <p>
                            <strong>Logged in</strong> on {dateFormat(session.created_at)}
                        </p>
                        <p>
                            <strong>Expires</strong> om {dateFormat(session.expires_at)}
                        </p>
                    </div>
                </div>
            ))}
        </div>
    );
}
