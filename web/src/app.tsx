/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { LocationProvider, Route, Router } from 'preact-iso';
import { Home } from './pages/home.tsx';
import { About } from './pages/about.tsx';
import { Menu } from './components/menu.tsx';

export function App() {
    return (
        <>
            <LocationProvider>
                <Menu />

                <Router>
                    <Route path="/" component={Home} />
                    <Route path="/about" component={About} />
                    <Route default component={() => <div>404 Not Found</div>} />
                </Router>
            </LocationProvider>
        </>
    );
}
