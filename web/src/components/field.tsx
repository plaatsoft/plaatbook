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
    required,
    autofocus,
    addonPre,
    addonPost,
}: {
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
    required?: boolean;
    autofocus?: boolean;
    addonPre?: JSX.Element;
    addonPost?: JSX.Element;
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

    const name = (placeholder || label)!.toLowerCase().replace(/ /g, '_');
    const control =
        type === 'textarea' ? (
            <textarea
                ref={textarea}
                id={name}
                className={`textarea ${error !== undefined ? 'is-danger' : ''}`}
                placeholder={placeholder || label}
                rows={rows || 3}
                value={value}
                onInput={
                    onInput !== undefined ? onInput : (event) => setValue!((event.target as HTMLTextAreaElement).value)
                }
                required={required}
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
                required={required}
                disabled={disabled}
            />
        );

    return (
        <div className="field">
            {label !== undefined && (
                <label className="label" htmlFor={name}>
                    {label}
                </label>
            )}
            {addonPre || addonPost ? (
                <div className="field has-addons">
                    {addonPre !== undefined && <div className="control">{addonPre}</div>}
                    <div className="control is-expanded">{control}</div>
                    {addonPost !== undefined && <div className="control">{addonPost}</div>}
                </div>
            ) : (
                control
            )}
            {help !== undefined && <p className="help">{help}</p>}
            {error !== undefined && error !== '' && <p className="help is-danger">{error}</p>}
        </div>
    );
}
