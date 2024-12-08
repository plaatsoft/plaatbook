/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useEffect } from 'preact/hooks';
import { LocationProvider, Route, Router, useLocation } from 'preact-iso';
import { Menu } from './components/menu.tsx';
import { Login } from './pages/auth/login.tsx';
import { Register } from './pages/auth/register.tsx';
import { Home } from './pages/home.tsx';
import { Settings } from './pages/settings.tsx';
import { AuthService, $authUser } from './services/auth.service.ts';

export function App() {
    const location = useLocation();

    // Auth user
    const auth = async () => {
        await AuthService.getInstance().auth(location);
    };
    useEffect(() => {
        auth();
    }, []);

    return (
        <LocationProvider>
            <Menu />

            {$authUser.value !== undefined && (
                <Router>
                    <Route path="/" component={Home} />
                    <Route path="/settings" component={Settings} />
                    <Route path="/auth/login" component={Login} />
                    <Route path="/auth/register" component={Register} />
                    <Route default component={() => <div>404 Not Found</div>} />
                </Router>
            )}
        </LocationProvider>
    );
}
