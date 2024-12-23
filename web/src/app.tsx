/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useEffect, useState } from 'preact/hooks';
// eslint-disable-next-line import/named
import { Router, Route, RouterOnChangeArgs } from 'preact-router';
import { Menu } from './components/menu.tsx';
import { Login } from './pages/auth/login.tsx';
import { Register } from './pages/auth/register.tsx';
import { Home } from './pages/home.tsx';
import { Settings } from './pages/settings.tsx';
import { AuthService, $authUser } from './services/auth.service.ts';
import { NotFound } from './pages/not-found.tsx';
import { UsersShow } from './pages/users/show.tsx';
import { Search } from './pages/search.tsx';
import { PostsShow } from './pages/posts/show.tsx';

export function App() {
    const [route, setRoute] = useState<RouterOnChangeArgs | null>(null);

    // Auth user
    const auth = async () => {
        await AuthService.getInstance().auth();
    };
    useEffect(() => {
        auth();
    }, []);

    return (
        <>
            <Menu route={route} />

            {$authUser.value !== undefined && $authUser.value !== null && (
                <Router onChange={setRoute}>
                    <Route path="/" component={Home} />
                    <Route path="/search" component={Search} />
                    <Route path="/posts/:post_id" component={PostsShow} />
                    <Route path="/users/:user_id" component={UsersShow} />
                    <Route path="/settings" component={Settings} />
                    <Route default component={NotFound} />
                </Router>
            )}
            {$authUser.value !== undefined && $authUser.value === null && (
                <Router onChange={setRoute}>
                    <Route path="/" component={Home} />
                    <Route path="/search" component={Search} />
                    <Route path="/posts/:post_id" component={PostsShow} />
                    <Route path="/users/:user_id" component={UsersShow} />
                    <Route path="/auth/login" component={Login} />
                    <Route path="/auth/register" component={Register} />
                    <Route default component={NotFound} />
                </Router>
            )}
        </>
    );
}
