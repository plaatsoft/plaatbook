/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useEffect, useState } from 'preact/hooks';
import { Field } from '../../components/forms/field.tsx';
import { AuthService } from '../../services/auth.service.ts';
import { Report } from '../../api.ts';
import { RegisterIcon } from '../../components/icons.tsx';
import { route } from '../../router.tsx';

export function Register() {
    const [isLoading, setIsLoading] = useState(false);
    const [username, setUsername] = useState('');
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [confirmPassword, setConfirmPassword] = useState('');
    const [report, setReport] = useState<Report>({});

    useEffect(() => {
        document.title = 'Register - PlaatBook';
    }, []);

    const register = async (event: SubmitEvent) => {
        event.preventDefault();
        if (password !== confirmPassword) {
            setReport({ confirm_password: ['Passwords do not match'] });
            return;
        }
        setIsLoading(true);
        setReport({});
        const report = await AuthService.getInstance().register(username, email, password);
        if (report === null) {
            await AuthService.getInstance().login(email, password);
            route('/');
        } else {
            if (password !== confirmPassword) {
                report.confirm_password = ['Passwords do not match'];
            }
            setIsLoading(false);
            setReport(report);
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
                error={report.username?.join(', ')}
                disabled={isLoading}
                autofocus
            />

            <Field
                type="email"
                label="Email address"
                value={email}
                setValue={setEmail}
                error={report.email?.join(', ')}
                disabled={isLoading}
            />

            <Field
                type="password"
                label="Password"
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
                    <RegisterIcon className="mr-2" />
                    Register
                </button>
            </div>
        </form>
    );
}
