import globals from 'globals';
import pluginJs from '@eslint/js';
// eslint-disable-next-line import/no-unresolved
import tseslint from 'typescript-eslint';
import pluginReact from 'eslint-plugin-react';
import pluginImport from 'eslint-plugin-import';

export default [
    { files: ['**/*.{js,mjs,cjs,ts,jsx,tsx}'] },
    { languageOptions: { globals: globals.browser } },
    pluginJs.configs.recommended,
    ...tseslint.configs.recommended,
    pluginImport.flatConfigs.recommended,
    pluginReact.configs.flat.recommended,
    // Ignore dist folder
    { ignores: ['dist'] },
    // Allow Preact instead of React
    { settings: { react: { version: '18.3.0' } } },
    { rules: { 'react/react-in-jsx-scope': 'off' } },
];
