/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

// Allow importing .scss files
declare module '*.scss';

// Allow using css template literal tag
declare function css(template: TemplateStringsArray): { [className: string]: string };

// .env variables
interface ImportMeta {
    readonly env: ImportMetaEnv;
}

interface ImportMetaEnv {
    readonly VITE_API_URL: string;
}
