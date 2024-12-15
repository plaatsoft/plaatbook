/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

// eslint-disable-next-line import/named
import { FunctionComponent, render } from 'preact';
import { ConfirmModal } from '../components/modals/confirm-modal.tsx';

export class DialogService {
    static instance?: DialogService;
    modalContainer: HTMLElement;

    constructor() {
        this.modalContainer = document.getElementById('modal-container')!;
    }

    static getInstance(): DialogService {
        if (DialogService.instance === undefined) {
            DialogService.instance = new DialogService();
        }
        return DialogService.instance;
    }

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    async open<T>(component: any, props: object): Promise<T> {
        return new Promise((resolve) => {
            const onConfirm = (result: T) => {
                document.documentElement.classList.remove('is-clipped');
                render(null, this.modalContainer);
                resolve(result);
            };
            document.documentElement.classList.add('is-clipped');
            const Component = component as FunctionComponent;
            // @ts-expect-error Preact types are not up to date
            render(<Component {...props} onConfirm={onConfirm} />, this.modalContainer);
        });
    }

    async confirm(title: string, message: string, action: string): Promise<boolean> {
        return this.open<boolean>(ConfirmModal, { title, message, action });
    }
}
