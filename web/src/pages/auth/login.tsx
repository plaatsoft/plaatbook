/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useEffect } from 'preact/hooks';
import { LoginForm } from '../../components/forms/login-form.tsx';

export function Login() {
    useEffect(() => {
        document.title = 'Login - PlaatBook';
    }, []);

    return <LoginForm />;
}
