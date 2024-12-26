/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

import { User } from '../../api.ts';
import { UserEditForm } from '../forms/user-edit-form.tsx';

export function UserEditDialog({ user, onConfirm }: { user: User; onConfirm: (updatedUser: User | null) => void }) {
    return (
        <div className="modal is-active">
            <div className="modal-background" onClick={() => onConfirm(null)}></div>
            <UserEditForm user={user} dialog onConfirm={onConfirm} />
        </div>
    );
}
