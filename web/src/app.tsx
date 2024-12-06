/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { LocationProvider, Route, Router, useLocation } from 'preact-iso';
import { Home } from './pages/home.tsx';
import { About } from './pages/about.tsx';
import { Menu } from './components/menu.tsx';
import { Login } from './pages/auth/login.tsx';
import { Register } from './pages/auth/register.tsx';
import { useEffect } from 'preact/hooks';
import { AuthService } from './services/auth.service.ts';

export function App() {
    const location = useLocation();

    // Auth user
    const auth = async () => {
        if ((await AuthService.getInstance().auth()) === 'logout') {
            location.route('/');
        }
    };
    useEffect(() => {
        auth();
    }, []);

    return (
        <LocationProvider>
            <Menu />

            <Router>
                <Route path="/" component={Home} />
                <Route path="/about" component={About} />
                <Route path="/auth/login" component={Login} />
                <Route path="/auth/register" component={Register} />
                <Route default component={() => <div>404 Not Found</div>} />
            </Router>
        </LocationProvider>
    );
}
