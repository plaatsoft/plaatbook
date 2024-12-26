/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useState, useEffect } from 'preact/hooks';
import { $authSession, $authUser, AuthService } from '../services/auth.service.ts';
import { Report, Session } from '../api.ts';
import { Field } from '../components/forms/field.tsx';
import { Notification } from '../components/notification.tsx';
import { dateFormat } from '../utils.ts';
import { DialogService } from '../services/dialog.service.tsx';
import { DeleteIcon, KeyIcon, OptionsIcon, SecurityEditIcon } from '../components/icons.tsx';
import { UserEditForm } from '../components/forms/user-edit-form.tsx';

export function Settings() {
    useEffect(() => {
        document.title = 'Settings - PlaatBook';
    }, []);

    return (
        <div className="section">
            <h2 className="title">Settings</h2>
            <UserEditForm user={$authUser.value!} />
            <ChangePasswordForm />
            <SessionsManagement />
        </div>
    );
}

function ChangePasswordForm() {
    const authService = AuthService.getInstance();
    const [isLoading, setIsLoading] = useState(false);
    const [isDone, setIsDone] = useState(false);
    const [currentPassword, setCurrentPassword] = useState('');
    const [password, setPassword] = useState('');
    const [confirmPassword, setConfirmPassword] = useState('');
    const [report, setReport] = useState<Report>({});

    const changePassword = async (event: SubmitEvent) => {
        event.preventDefault();
        if (password !== confirmPassword) {
            setReport({ confirm_password: ['Passwords do not match'] });
            return;
        }
        setIsLoading(true);
        setReport({});
        const report = await authService.changePassword(currentPassword, password);
        setIsLoading(false);
        if (report === null) {
            setIsDone(true);
        } else {
            if (password !== confirmPassword) {
                report.confirm_password = ['Passwords do not match'];
            }
            setReport(report);
        }
    };

    return (
        <form className="box" onSubmit={changePassword}>
            <h2 className="title is-5">Change account password</h2>

            {isDone && <Notification text="New password saved" />}

            <Field
                type="password"
                label="Current password"
                value={currentPassword}
                setValue={setCurrentPassword}
                error={report.current_password?.join(', ')}
                disabled={isLoading}
            />

            <Field
                type="password"
                label="New password"
                value={password}
                setValue={setPassword}
                error={report.password?.join(', ')}
                disabled={isLoading}
            />

            <Field
                type="password"
                label="Confirm new password"
                value={confirmPassword}
                setValue={setConfirmPassword}
                disabled={isLoading}
                error={report.confirm_password?.join(', ')}
            />

            <div className="field">
                <button type="submit" className="button is-link">
                    <SecurityEditIcon className="mr-2" />
                    Change password
                </button>
            </div>
        </form>
    );
}

function SessionsManagement() {
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
            if (await AuthService.getInstance().revokeSession(session)) {
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
