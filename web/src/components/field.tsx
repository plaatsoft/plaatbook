/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { createRef } from 'preact';
// eslint-disable-next-line import/named
import { Dispatch, StateUpdater, useEffect } from 'preact/hooks';

export function Field({
    name,
    type,
    label,
    placeholder,
    rows,
    value,
    onInput,
    setValue,
    error,
    disabled,
    autofocus,
}: {
    name: string;
    type: string;
    label?: string;
    placeholder?: string;
    rows?: number;
    value: string;
    onInput?: (event: InputEvent) => void;
    setValue?: Dispatch<StateUpdater<string>>;
    error?: string;
    disabled?: boolean;
    autofocus?: boolean;
}) {
    const input = createRef<HTMLInputElement>();

    if (autofocus) {
        useEffect(() => {
            input.current!.focus();
        }, []);
    }

    return (
        <div className="field">
            {label !== undefined && (
                <label className="label" htmlFor={name}>
                    {label}
                </label>
            )}
            {type === 'textarea' ? (
                <textarea
                    className={`textarea ${error !== undefined ? 'is-danger' : ''}`}
                    placeholder={placeholder || label}
                    rows={rows || 3}
                    value={value}
                    onInput={
                        onInput !== undefined
                            ? onInput
                            : (event) => setValue!((event.target as HTMLTextAreaElement).value)
                    }
                    disabled={disabled}
                />
            ) : (
                <input
                    ref={input}
                    id={name}
                    className={`input ${error !== undefined ? 'is-danger' : ''}`}
                    type={type}
                    placeholder={placeholder || label}
                    value={value}
                    onInput={
                        onInput !== undefined ? onInput : (event) => setValue!((event.target as HTMLInputElement).value)
                    }
                    disabled={disabled}
                />
            )}
            {error !== undefined && error !== '' && <p className="help is-danger">{error}</p>}
        </div>
    );
}
