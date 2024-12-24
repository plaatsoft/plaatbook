/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

// eslint-disable-next-line import/named
import { FunctionComponent, render } from 'preact';
import { ConfirmDialog } from '../components/dialogs/confirm-dialog.tsx';

export class DialogService {
    static instance?: DialogService;
    dialogContainer: HTMLElement;

    constructor() {
        this.dialogContainer = document.getElementById('dialog-container')!;
    }

    static getInstance(): DialogService {
        if (DialogService.instance === undefined) {
            DialogService.instance = new DialogService();
        }
        return DialogService.instance;
    }

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    async open<T>(component: any, props = {}): Promise<T> {
        return new Promise((resolve) => {
            const onConfirm = (result: T) => {
                document.documentElement.classList.remove('is-clipped');
                render(null, this.dialogContainer);
                resolve(result);
            };
            document.documentElement.classList.add('is-clipped');
            const Component = component as FunctionComponent;
            // @ts-expect-error Preact types are not up to date
            render(<Component {...props} onConfirm={onConfirm} />, this.dialogContainer);
        });
    }

    async confirm(
        title: string,
        message: string,
        action: string,
        ActionIcon?: FunctionComponent<{ className: string }>,
    ): Promise<boolean> {
        return this.open<boolean>(ConfirmDialog, { title, message, action, ActionIcon });
    }
}
