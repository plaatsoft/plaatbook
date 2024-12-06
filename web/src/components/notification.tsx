/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

export function Notification({ text, color }: { text: string; color?: string }) {
    const remove = (event: MouseEvent) => {
        const target = event.target as HTMLButtonElement;
        target.parentNode!.parentNode!.removeChild(target.parentNode!);
    };
    return (
        <div className={`notification is-${color ?? 'link'}`}>
            <button className="delete" onClick={remove}></button>
            {text}
        </div>
    );
}
