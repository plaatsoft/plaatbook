/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

export interface User {
    id: string;
    username: string;
    email: string;
    firstname?: string;
    lastname?: string;
    birthdate?: string;
    bio?: string;
    location?: string;
    website?: string;
    created_at: string;
    updated_at: string;
}
