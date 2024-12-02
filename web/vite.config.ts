import { defineConfig } from 'vite';
// eslint-disable-next-line import/no-named-as-default
import preact from '@preact/preset-vite';
import inlineCssModules from 'vite-plugin-inline-css-modules';

export default defineConfig({
    plugins: [preact(), inlineCssModules()],

    // Disable warnings in Bulma
    css: {
        preprocessorOptions: {
            scss: {
                quietDeps: true,
            },
        },
    },
});
