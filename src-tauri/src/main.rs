#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod audio;
mod vad;
mod stt;

use std::sync::Mutex;
use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, State,
};

struct AppState {
    is_recording: Mutex<bool>,
}

impl AppState {
    fn new() -> Self {
        Self {
            is_recording: Mutex::new(false),
        }
    }
}

#[tauri::command]
fn start_capture(state: State<AppState>) -> Result<String, String> {
    let mut is_recording = state.is_recording.lock().map_err(|e| e.to_string())?;
    if *is_recording {
        return Ok("Already recording".to_string());
    }
    *is_recording = true;
    // TODO: Initialize audio capture
    Ok("Capture started".to_string())
}

#[tauri::command]
fn stop_capture(state: State<AppState>) -> Result<String, String> {
    let mut is_recording = state.is_recording.lock().map_err(|e| e.to_string())?;
    if !*is_recording {
        return Ok("Not recording".to_string());
    }
    *is_recording = false;
    // TODO: Stop audio capture
    Ok("Capture stopped".to_string())
}

#[tauri::command]
fn get_capture_status(state: State<AppState>) -> Result<bool, String> {
    let is_recording = state.is_recording.lock().map_err(|e| e.to_string())?;
    Ok(*is_recording)
}

fn build_tray_menu(app: &AppHandle) -> SystemTray {
    let start = CustomMenuItem::new("start".to_string(), "🎙️ Start Capture");
    let stop = CustomMenuItem::new("stop".to_string(), "⏹️ Stop Capture");
    let separator = SystemTrayMenuItem::Separator;
    let show = CustomMenuItem::new("show".to_string(), "Show Window");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");

    SystemTray::new()
        .with_menu(
            SystemTrayMenu::new()
                .add_item(start)
                .add_item(stop)
                .add_native_item(separator)
                .add_item(show)
                .add_native_item(SystemTrayMenuItem::Separator)
                .add_item(quit),
        )
        .with_tooltip("🎙️ Noter — Idle")
}

fn update_tray_tooltip(app: &AppHandle, is_recording: bool) {
    let tooltip = if is_recording {
        "🎙️ Noter — Recording"
    } else {
        "🎙️ Noter — Idle"
    };
    if let Some(tray) = app.tray_by_id("main") {
        let _ = tray.set_tooltip(Some(tooltip));
    }
}

fn main() {
    env_logger::init();

    tauri::Builder::default()
        .system_tray(build_tray_menu)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "show" => {
                    if let Some(window) = app.get_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                "start" => {
                    if let Some(state) = app.try_state::<AppState>() {
                        if let Ok(mut is_recording) = state.is_recording.lock() {
                            if !*is_recording {
                                *is_recording = true;
                                update_tray_tooltip(app, true);
                                let _ = app.emit("capture-status-changed", true);
                            }
                        }
                    }
                }
                "stop" => {
                    if let Some(state) = app.try_state::<AppState>() {
                        if let Ok(mut is_recording) = state.is_recording.lock() {
                            if *is_recording {
                                *is_recording = false;
                                update_tray_tooltip(app, false);
                                let _ = app.emit("capture-status-changed", false);
                            }
                        }
                    }
                }
                _ => {}
            },
            SystemTrayEvent::LeftClick { .. } => {
                if let Some(window) = app.get_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            _ => {}
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            start_capture,
            stop_capture,
            get_capture_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
