/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

// eslint-disable-next-line import/named
import { JSX } from 'preact';
// eslint-disable-next-line import/named
import { Dispatch, StateUpdater, useEffect, useRef } from 'preact/hooks';

export function Field({
    name,
    type,
    label,
    placeholder,
    rows,
    value,
    onInput,
    setValue,
    help,
    error,
    disabled,
    autofocus,
    addon,
    expanded,
}: {
    name: string;
    type: string;
    label?: string;
    placeholder?: string;
    rows?: number;
    value: string;
    onInput?: (event: InputEvent) => void;
    setValue?: Dispatch<StateUpdater<string>>;
    help?: JSX.Element;
    error?: string;
    disabled?: boolean;
    autofocus?: boolean;
    addon?: boolean;
    expanded?: boolean;
}) {
    const textarea = useRef<HTMLTextAreaElement>(null);
    const input = useRef<HTMLInputElement>(null);

    if (autofocus) {
        useEffect(() => {
            if (type === 'textarea') {
                textarea.current!.focus();
            } else {
                input.current!.focus();
            }
        }, []);
    }

    return (
        <div className={`${addon ? 'control' : 'field'} ${expanded ? 'is-expanded' : ''}`}>
            {label !== undefined && (
                <label className="label" htmlFor={name}>
                    {label}
                </label>
            )}
            {type === 'textarea' ? (
                <textarea
                    ref={textarea}
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
            {help !== undefined && <p className="help">{help}</p>}
            {error !== undefined && error !== '' && <p className="help is-danger">{error}</p>}
        </div>
    );
}
