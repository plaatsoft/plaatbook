/*
 * Copyright (c) 2024 PlaatSoft
 *
 * SPDX-License-Identifier: MIT
 */

export function AppIcon({ className }: { className?: string }) {
    return (
        <svg className={`icon ${className}`} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path d="M18 2H12V9L9.5 7.5L7 9V2H6A2 2 0 0 0 4 4V20A2 2 0 0 0 6 22H18A2 2 0 0 0 20 20V4A2 2 0 0 0 18 2M14 12A2 2 0 1 1 12 14A2 2 0 0 1 14 12M18 20H10V19C10 17.67 12.67 17 14 17S18 17.67 18 19Z" />
        </svg>
    );
}

export function SearchIcon({ className }: { className?: string }) {
    return (
        <svg className={`icon ${className}`} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path d="M9.5,3A6.5,6.5 0 0,1 16,9.5C16,11.11 15.41,12.59 14.44,13.73L14.71,14H15.5L20.5,19L19,20.5L14,15.5V14.71L13.73,14.44C12.59,15.41 11.11,16 9.5,16A6.5,6.5 0 0,1 3,9.5A6.5,6.5 0 0,1 9.5,3M9.5,5C7,5 5,7 5,9.5C5,12 7,14 9.5,14C12,14 14,12 14,9.5C14,7 12,5 9.5,5Z" />
        </svg>
    );
}

export function AccountIcon({ className }: { className?: string }) {
    return (
        <svg className={`icon ${className}`} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path d="M12,4A4,4 0 0,1 16,8A4,4 0 0,1 12,12A4,4 0 0,1 8,8A4,4 0 0,1 12,4M12,14C16.42,14 20,15.79 20,18V20H4V18C4,15.79 7.58,14 12,14Z" />
        </svg>
    );
}

export function SettingsIcon({ className }: { className?: string }) {
    return (
        <svg className={`icon ${className}`} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path d="M12,15.5A3.5,3.5 0 0,1 8.5,12A3.5,3.5 0 0,1 12,8.5A3.5,3.5 0 0,1 15.5,12A3.5,3.5 0 0,1 12,15.5M19.43,12.97C19.47,12.65 19.5,12.33 19.5,12C19.5,11.67 19.47,11.34 19.43,11L21.54,9.37C21.73,9.22 21.78,8.95 21.66,8.73L19.66,5.27C19.54,5.05 19.27,4.96 19.05,5.05L16.56,6.05C16.04,5.66 15.5,5.32 14.87,5.07L14.5,2.42C14.46,2.18 14.25,2 14,2H10C9.75,2 9.54,2.18 9.5,2.42L9.13,5.07C8.5,5.32 7.96,5.66 7.44,6.05L4.95,5.05C4.73,4.96 4.46,5.05 4.34,5.27L2.34,8.73C2.21,8.95 2.27,9.22 2.46,9.37L4.57,11C4.53,11.34 4.5,11.67 4.5,12C4.5,12.33 4.53,12.65 4.57,12.97L2.46,14.63C2.27,14.78 2.21,15.05 2.34,15.27L4.34,18.73C4.46,18.95 4.73,19.03 4.95,18.95L7.44,17.94C7.96,18.34 8.5,18.68 9.13,18.93L9.5,21.58C9.54,21.82 9.75,22 10,22H14C14.25,22 14.46,21.82 14.5,21.58L14.87,18.93C15.5,18.67 16.04,18.34 16.56,17.94L19.05,18.95C19.27,19.03 19.54,18.95 19.66,18.73L21.66,15.27C21.78,15.05 21.73,14.78 21.54,14.63L19.43,12.97Z" />
        </svg>
    );
}

export function LoginIcon({ className }: { className?: string }) {
    return (
        <svg className={`icon ${className}`} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path d="M11 7L9.6 8.4L12.2 11H2V13H12.2L9.6 15.6L11 17L16 12L11 7M20 19H12V21H20C21.1 21 22 20.1 22 19V5C22 3.9 21.1 3 20 3H12V5H20V19Z" />
        </svg>
    );
}

export function RegisterIcon({ className }: { className?: string }) {
    return (
        <svg className={`icon ${className}`} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path d="M15,14C12.33,14 7,15.33 7,18V20H23V18C23,15.33 17.67,14 15,14M6,10V7H4V10H1V12H4V15H6V12H9V10M15,12A4,4 0 0,0 19,8A4,4 0 0,0 15,4A4,4 0 0,0 11,8A4,4 0 0,0 15,12Z" />
        </svg>
    );
}

export function LogoutIcon({ className }: { className?: string }) {
    return (
        <svg className={`icon ${className}`} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path d="M17 7L15.59 8.41L18.17 11H8V13H18.17L15.59 15.58L17 17L22 12M4 5H12V3H4C2.9 3 2 3.9 2 5V19C2 20.1 2.9 21 4 21H12V19H4V5Z" />
        </svg>
    );
}

export function OptionsIcon({ className }: { className?: string }) {
    return (
        <svg className={`icon ${className}`} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path d="M12,16A2,2 0 0,1 14,18A2,2 0 0,1 12,20A2,2 0 0,1 10,18A2,2 0 0,1 12,16M12,10A2,2 0 0,1 14,12A2,2 0 0,1 12,14A2,2 0 0,1 10,12A2,2 0 0,1 12,10M12,4A2,2 0 0,1 14,6A2,2 0 0,1 12,8A2,2 0 0,1 10,6A2,2 0 0,1 12,4Z" />
        </svg>
    );
}

export function CommentIcon({ className }: { className?: string }) {
    return (
        <svg className={`icon ${className}`} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path d="M12,3C17.5,3 22,6.58 22,11C22,15.42 17.5,19 12,19C10.76,19 9.57,18.82 8.47,18.5C5.55,21 2,21 2,21C4.33,18.67 4.7,17.1 4.75,16.5C3.05,15.07 2,13.13 2,11C2,6.58 6.5,3 12,3Z" />
        </svg>
    );
}

export function EditIcon({ className }: { className?: string }) {
    return (
        <svg className={`icon ${className}`} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path d="M20.71,7.04C21.1,6.65 21.1,6 20.71,5.63L18.37,3.29C18,2.9 17.35,2.9 16.96,3.29L15.12,5.12L18.87,8.87M3,17.25V21H6.75L17.81,9.93L14.06,6.18L3,17.25Z" />
        </svg>
    );
}

export function DeleteIcon({ className }: { className?: string }) {
    return (
        <svg className={`icon ${className}`} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path d="M19,4H15.5L14.5,3H9.5L8.5,4H5V6H19M6,19A2,2 0 0,0 8,21H16A2,2 0 0,0 18,19V7H6V19Z" />
        </svg>
    );
}

export function RepostIcon({ className }: { className?: string }) {
    return (
        <svg className={`icon ${className}`} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path d="M17,17H7V14L3,18L7,22V19H19V13H17M7,7H17V10L21,6L17,2V5H5V11H7V7Z" />
        </svg>
    );
}

export function LikeIcon({ className }: { className?: string }) {
    return (
        <svg className={`icon ${className}`} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path d="M23,10C23,8.89 22.1,8 21,8H14.68L15.64,3.43C15.66,3.33 15.67,3.22 15.67,3.11C15.67,2.7 15.5,2.32 15.23,2.05L14.17,1L7.59,7.58C7.22,7.95 7,8.45 7,9V19A2,2 0 0,0 9,21H18C18.83,21 19.54,20.5 19.84,19.78L22.86,12.73C22.95,12.5 23,12.26 23,12V10M1,21H5V9H1V21Z" />
        </svg>
    );
}

export function DislikeIcon({ className }: { className?: string }) {
    return (
        <svg className={`icon ${className}`} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path d="M19,15H23V3H19M15,3H6C5.17,3 4.46,3.5 4.16,4.22L1.14,11.27C1.05,11.5 1,11.74 1,12V14A2,2 0 0,0 3,16H9.31L8.36,20.57C8.34,20.67 8.33,20.77 8.33,20.88C8.33,21.3 8.5,21.67 8.77,21.94L9.83,23L16.41,16.41C16.78,16.05 17,15.55 17,15V5C17,3.89 16.1,3 15,3Z" />
        </svg>
    );
}

export function StatsIcon({ className }: { className?: string }) {
    return (
        <svg className={`icon ${className}`} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path d="M22,21H2V3H4V19H6V10H10V19H12V6H16V19H18V14H22V21Z" />
        </svg>
    );
}

export function AccountEditIcon({ className }: { className?: string }) {
    return (
        <svg className={`icon ${className}`} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path d="M21.7,13.35L20.7,14.35L18.65,12.3L19.65,11.3C19.86,11.09 20.21,11.09 20.42,11.3L21.7,12.58C21.91,12.79 21.91,13.14 21.7,13.35M12,18.94L18.06,12.88L20.11,14.93L14.06,21H12V18.94M12,14C7.58,14 4,15.79 4,18V20H10V18.11L14,14.11C13.34,14.03 12.67,14 12,14M12,4A4,4 0 0,0 8,8A4,4 0 0,0 12,12A4,4 0 0,0 16,8A4,4 0 0,0 12,4Z" />
        </svg>
    );
}

export function SecurityEditIcon({ className }: { className?: string }) {
    return (
        <svg className={`icon ${className}`} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path d="M21.7 14.4L20.7 15.4L18.6 13.3L19.6 12.3C19.8 12.1 20.2 12.1 20.4 12.3L21.7 13.6C21.9 13.8 21.9 14.1 21.7 14.4M12 19.9L18.1 13.8L20.2 15.9L14.1 22H12V19.9M10 19.1L21 8.1V5L12 1L3 5V11C3 15.8 5.9 20.3 10 22.3V19.1Z" />
        </svg>
    );
}

export function KeyIcon({ className }: { className?: string }) {
    return (
        <svg className={`icon ${className}`} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path d="M22,18V22H18V19H15V16H12L9.74,13.74C9.19,13.91 8.61,14 8,14A6,6 0 0,1 2,8A6,6 0 0,1 8,2A6,6 0 0,1 14,8C14,8.61 13.91,9.19 13.74,9.74L22,18M7,5A2,2 0 0,0 5,7A2,2 0 0,0 7,9A2,2 0 0,0 9,7A2,2 0 0,0 7,5Z" />
        </svg>
    );
}

export function ShareIcon({ className }: { className?: string }) {
    return (
        <svg className={`icon ${className}`} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path d="M21,12L14,5V9C7,10 4,15 3,20C5.5,16.5 9,14.9 14,14.9V19L21,12Z" />
        </svg>
    );
}

export function CopyIcon({ className }: { className?: string }) {
    return (
        <svg className={`icon ${className}`} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path d="M19,21H8V7H19M19,5H8A2,2 0 0,0 6,7V21A2,2 0 0,0 8,23H19A2,2 0 0,0 21,21V7A2,2 0 0,0 19,5M16,1H4A2,2 0 0,0 2,3V17H4V3H16V1Z" />
        </svg>
    );
}

export function LocationIcon({ className }: { className?: string }) {
    return (
        <svg className={`icon ${className}`} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path d="M12,11.5A2.5,2.5 0 0,1 9.5,9A2.5,2.5 0 0,1 12,6.5A2.5,2.5 0 0,1 14.5,9A2.5,2.5 0 0,1 12,11.5M12,2A7,7 0 0,0 5,9C5,14.25 12,22 12,22C12,22 19,14.25 19,9A7,7 0 0,0 12,2Z" />
        </svg>
    );
}

export function BirthdateIcon({ className }: { className?: string }) {
    return (
        <svg className={`icon ${className}`} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path d="M11.5,0.5C12,0.75 13,2.4 13,3.5C13,4.6 12.33,5 11.5,5C10.67,5 10,4.85 10,3.75C10,2.65 11,2 11.5,0.5M18.5,9C21,9 23,11 23,13.5C23,15.06 22.21,16.43 21,17.24V23H12L3,23V17.24C1.79,16.43 1,15.06 1,13.5C1,11 3,9 5.5,9H10V6H13V9H18.5M12,16A2.5,2.5 0 0,0 14.5,13.5H16A2.5,2.5 0 0,0 18.5,16A2.5,2.5 0 0,0 21,13.5A2.5,2.5 0 0,0 18.5,11H5.5A2.5,2.5 0 0,0 3,13.5A2.5,2.5 0 0,0 5.5,16A2.5,2.5 0 0,0 8,13.5H9.5A2.5,2.5 0 0,0 12,16Z" />
        </svg>
    );
}

export function CalendarIcon({ className }: { className?: string }) {
    return (
        <svg className={`icon ${className}`} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path d="M12 9C14 9 15 11.42 13.59 12.84C12.17 14.26 9.75 13.25 9.75 11.25C9.75 10 10.75 9 12 9M16.5 18H7.5V16.88C7.5 15.63 9.5 14.63 12 14.63S16.5 15.63 16.5 16.88M19 19H5V8H19M16 1V3H8V1H6V3H5C3.9 3 3 3.9 3 5V19C3 20.11 3.9 21 5 21H19C20.11 21 21 20.11 21 19V5C21 3.9 20.11 3 19 3H18V1H16Z" />
        </svg>
    );
}

export function LinkIcon({ className }: { className?: string }) {
    return (
        <svg className={`icon ${className}`} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path d="M3.9,12C3.9,10.29 5.29,8.9 7,8.9H11V7H7A5,5 0 0,0 2,12A5,5 0 0,0 7,17H11V15.1H7C5.29,15.1 3.9,13.71 3.9,12M8,13H16V11H8V13M17,7H13V8.9H17C18.71,8.9 20.1,10.29 20.1,12C20.1,13.71 18.71,15.1 17,15.1H13V17H17A5,5 0 0,0 22,12A5,5 0 0,0 17,7Z" />
        </svg>
    );
}
