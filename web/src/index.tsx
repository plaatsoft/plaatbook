/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { render } from 'preact';
import { App } from './app.tsx';
import './index.scss';

render(<App />, document.getElementById('app')!);
