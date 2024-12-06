/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

export interface Session {
    id: string;
    user_id: string;
    token: string;
    ip_address: string;
    ip_latitude?: number;
    ip_longitude?: number;
    ip_country?: string;
    ip_city?: string;
    client_name?: string;
    client_version?: string;
    client_os?: string;
    expires_at: Date;
    created_at: Date;
    updated_at: Date;
}
