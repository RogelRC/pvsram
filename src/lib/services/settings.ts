import { invoke } from "@tauri-apps/api/core";

export type WindowMode = "fullscreen" | "maximized" | "windowed";

export function getWindowMode(): Promise<WindowMode> {
    return invoke("get_window_mode");
}

export function setWindowMode(mode: WindowMode): Promise<void> {
    return invoke("set_window_mode", { mode });
}
