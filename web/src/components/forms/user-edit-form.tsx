/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { useEffect, useState } from 'preact/hooks';
import { Errors } from '../../models/errors.ts';
import { Field } from './field.tsx';
import { $authUser } from '../../services/auth.service.ts';
import { Notification } from '../notification.tsx';
import { AccountEditIcon } from '../icons.tsx';
import { User } from '../../models/user.ts';
import { UsersService } from '../../services/users.service.ts';

export function UserEditForm({
    user,
    dialog,
    onConfirm,
}: {
    user: User;
    dialog?: boolean;
    onConfirm?: (updatedUser: User | null) => void;
}) {
    const [isLoading, setIsLoading] = useState(false);
    const [username, setUsername] = useState(user.username ?? '');
    const [email, setEmail] = useState(user.email ?? '');
    const [firstname, setFirstname] = useState(user.firstname ?? '');
    const [lastname, setLastname] = useState(user.lastname ?? '');
    const [birthdate, setBirthdate] = useState(user.birthdate ?? '');
    const [bio, setBio] = useState(user.bio ?? '');
    const [location, setLocation] = useState(user.location ?? '');
    const [website, setWebsite] = useState(user.website ?? '');
    const [errors, setErrors] = useState<Errors>({});
    const [isDone, setIsDone] = useState(false);

    useEffect(() => {
        setUsername(user.username ?? '');
        setEmail(user.email ?? '');
        setFirstname(user.firstname ?? '');
        setLastname(user.lastname ?? '');
        setBirthdate(user.birthdate ?? '');
        setBio(user.bio ?? '');
        setLocation(user.location ?? '');
        setWebsite(user.website ?? '');
        setErrors({});
        setIsDone(false);
    }, [user]);

    const update = async (event: SubmitEvent) => {
        console.log('test');
        event.preventDefault();
        setIsLoading(true);
        setErrors({});
        const [sucess, result] = await UsersService.getInstance().update(user.id, {
            firstname,
            lastname,
            username,
            email,
            birthdate,
            bio,
            location,
            website,
        });
        setIsLoading(false);
        if (sucess) {
            if (user.id === $authUser.value!.id) {
                $authUser.value = result as User;
            }
            if (dialog) {
                console.log(result);
                onConfirm!(result as User);
            } else {
                setIsDone(true);
            }
        } else {
            setErrors(result as Errors);
        }
    };

    const fields = (
        <>
            <div className="columns">
                <div className="column">
                    <Field
                        type="text"
                        label="First name"
                        value={firstname}
                        setValue={setFirstname}
                        error={errors.firstname?.join(', ')}
                        disabled={isLoading}
                    />
                </div>
                <div className="column">
                    <Field
                        type="text"
                        label="Last name"
                        value={lastname}
                        setValue={setLastname}
                        error={errors.lastname?.join(', ')}
                        disabled={isLoading}
                    />
                </div>
            </div>

            <Field
                type="text"
                label="Username"
                value={username}
                setValue={setUsername}
                error={errors.username?.join(', ')}
                required
                disabled={isLoading}
                addonPre={<div className="button is-static">@</div>}
            />

            <Field
                type="email"
                label="Email address"
                value={email}
                setValue={setEmail}
                error={errors.email?.join(', ')}
                required
                disabled={isLoading}
            />

            <Field
                type="date"
                label="Birthdate"
                value={birthdate}
                setValue={setBirthdate}
                error={errors.birthdate?.join(', ')}
                disabled={isLoading}
            />

            <Field
                type="textarea"
                label="Bio"
                value={bio}
                setValue={setBio}
                error={errors.bio?.join(', ')}
                disabled={isLoading}
            />

            <div className="columns">
                <div className="column">
                    <Field
                        type="text"
                        label="Location"
                        value={location}
                        setValue={setLocation}
                        error={errors.location?.join(', ')}
                        disabled={isLoading}
                    />
                </div>
                <div className="column">
                    <Field
                        type="text"
                        label="Website"
                        value={website}
                        setValue={setWebsite}
                        error={errors.website?.join(', ')}
                        disabled={isLoading}
                    />
                </div>
            </div>
        </>
    );

    return dialog ? (
        <form className="modal-card" onSubmit={update}>
            <header className="modal-card-head">
                <p className="modal-card-title">Edit profile</p>
                <button type="button" className="delete" onClick={() => onConfirm!(null)}></button>
            </header>
            <section className="modal-card-body">{fields}</section>
            <footer className="modal-card-foot">
                <div className="buttons">
                    <button type="submit" className="button is-link">
                        <AccountEditIcon className="mr-2" />
                        Edit profile
                    </button>
                    <button type="button" className="button" onClick={() => onConfirm!(null)}>
                        Cancel
                    </button>
                </div>
            </footer>
        </form>
    ) : (
        <form className="box" onSubmit={update}>
            <h2 className="title is-5">Edit profile</h2>

            {isDone && <Notification text="Profile saved" />}

            {fields}

            <div className="field">
                <button type="submit" className="button is-link">
                    <AccountEditIcon className="mr-2" />
                    Edit profile
                </button>
            </div>
        </form>
    );
}
