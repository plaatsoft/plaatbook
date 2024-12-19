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
                    <svg className="icon mr-2" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                        <path d="M21.7,13.35L20.7,14.35L18.65,12.3L19.65,11.3C19.86,11.09 20.21,11.09 20.42,11.3L21.7,12.58C21.91,12.79 21.91,13.14 21.7,13.35M12,18.94L18.06,12.88L20.11,14.93L14.06,21H12V18.94M12,14C7.58,14 4,15.79 4,18V20H10V18.11L14,14.11C13.34,14.03 12.67,14 12,14M12,4A4,4 0 0,0 8,8A4,4 0 0,0 12,12A4,4 0 0,0 16,8A4,4 0 0,0 12,4Z" />
                    </svg>
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
                    <svg className="icon mr-2" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                        <path d="M21.7 14.4L20.7 15.4L18.6 13.3L19.6 12.3C19.8 12.1 20.2 12.1 20.4 12.3L21.7 13.6C21.9 13.8 21.9 14.1 21.7 14.4M12 19.9L18.1 13.8L20.2 15.9L14.1 22H12V19.9M10 19.1L21 8.1V5L12 1L3 5V11C3 15.8 5.9 20.3 10 22.3V19.1Z" />
                    </svg>
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
                                    <svg className="icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                                        <path d="M12,16A2,2 0 0,1 14,18A2,2 0 0,1 12,20A2,2 0 0,1 10,18A2,2 0 0,1 12,16M12,10A2,2 0 0,1 14,12A2,2 0 0,1 12,14A2,2 0 0,1 10,12A2,2 0 0,1 12,10M12,4A2,2 0 0,1 14,6A2,2 0 0,1 12,8A2,2 0 0,1 10,6A2,2 0 0,1 12,4Z" />
                                    </svg>
                                </button>
                            </div>
                            <div className="dropdown-menu">
                                <div className="dropdown-content">
                                    <a className="dropdown-item" href="#" onClick={() => revokeSession(session)}>
                                        <svg className="icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                                            <path d="M19,4H15.5L14.5,3H9.5L8.5,4H5V6H19M6,19A2,2 0 0,0 8,21H16A2,2 0 0,0 18,19V7H6V19Z" />
                                        </svg>
                                        Revoke session
                                    </a>
                                </div>
                            </div>
                        </div>

                        <h3 className="subtitle mb-2">
                            {session.client_name} on {session.client_os}
                            {session.id === $authSession.value!.id && (
                                <span className="tag ml-2">
                                    <svg
                                        className="icon is-small mr-1"
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 24 24"
                                    >
                                        <path d="M22,18V22H18V19H15V16H12L9.74,13.74C9.19,13.91 8.61,14 8,14A6,6 0 0,1 2,8A6,6 0 0,1 8,2A6,6 0 0,1 14,8C14,8.61 13.91,9.19 13.74,9.74L22,18M7,5A2,2 0 0,0 5,7A2,2 0 0,0 7,9A2,2 0 0,0 9,7A2,2 0 0,0 7,5Z" />
                                    </svg>
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
