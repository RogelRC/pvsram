import { invoke } from "@tauri-apps/api/core";

export function hasPassword(): Promise<boolean> {
    return invoke("has_password");
}

export function setInitialPassword(password: string): Promise<void> {
    return invoke("set_initial_password", { password });
}

export function login(password: string): Promise<boolean> {
    return invoke("login", { password });
}

export function logout(): Promise<void> {
    return invoke("logout");
}

export function isAuthenticated(): Promise<boolean> {
    return invoke("is_authenticated");
}

export function changePassword(params: {
    currentPassword: string;
    newPassword: string;
}): Promise<void> {
    return invoke("change_password", {
        currentPassword: params.currentPassword,
        newPassword: params.newPassword,
    });
}
