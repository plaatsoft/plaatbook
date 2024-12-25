/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useEffect } from 'preact/hooks';
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
import { Route } from './router.tsx';

export function App() {
    useEffect(() => {
        AuthService.getInstance().updateAuth();
    }, []);

    return (
        <>
            <Menu />

            {$authUser.value !== undefined && (
                <>
                    <Route path="/" component={Home} />
                    <Route path="/search" component={Search} />
                    <Route path="/posts/:post_id" component={PostsShow} />
                    <Route path="/users/:user_id" component={UsersShow} />
                    {$authUser.value !== null ? (
                        <>
                            <Route path="/settings" component={Settings} />
                        </>
                    ) : (
                        <>
                            <Route path="/auth/login" component={Login} />
                            <Route path="/auth/register" component={Register} />
                        </>
                    )}
                    <Route fallback component={NotFound} />
                </>
            )}
        </>
    );
}
