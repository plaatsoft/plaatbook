/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { signal } from '@preact/signals';

export const $route = signal(window.location.pathname);
let matches = false;

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function Route({ path, component, fallback }: { path?: string; component: any; fallback?: boolean }) {
    const Component = component;

    if (fallback) {
        if (!matches) {
            matches = true;
            return <Component />;
        }
        return;
    }

    const paramNames = path!.match(/:([^/]+)/g) || [];
    const match = $route.value.match(new RegExp(`^${path!.replace(/:([^/]+)/g, '([^/]+)')}$`));
    if (match && !matches) {
        const params: { [key: string]: string } = {};
        for (let i = 0; i < paramNames.length; i++) {
            params[paramNames[i].substring(1)] = match[i + 1];
        }
        matches = true;
        return <Component {...params} />;
    }
    return null;
}

export function route(to: string) {
    window.history.pushState({}, '', to);
    $route.value = new URL(to, window.location.origin).pathname;
    matches = false;
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function Link({ href, ...props }: any) {
    const open = (event: MouseEvent) => {
        event.preventDefault();
        event.stopPropagation();
        route(href);
    };
    return <a href={href} {...props} onClick={open} />;
}
