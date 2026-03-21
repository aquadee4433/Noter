#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod audio;
mod vad;
mod stt;
mod model;

use std::sync::Mutex;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, State,
};

struct AppState {
    is_recording: Mutex<bool>,
}

#[tauri::command]
fn start_capture(state: State<AppState>) -> Result<String, String> {
    let mut is_recording = state.is_recording.lock().map_err(|e| e.to_string())?;
    if *is_recording {
        return Ok("Already recording".to_string());
    }
    *is_recording = true;
    Ok("Capture started".to_string())
}

#[tauri::command]
fn stop_capture(state: State<AppState>) -> Result<String, String> {
    let mut is_recording = state.is_recording.lock().map_err(|e| e.to_string())?;
    if !*is_recording {
        return Ok("Not recording".to_string());
    }
    *is_recording = false;
    Ok("Capture stopped".to_string())
}

#[tauri::command]
fn get_capture_status(state: State<AppState>) -> Result<bool, String> {
    let is_recording = state.is_recording.lock().map_err(|e| e.to_string())?;
    Ok(*is_recording)
}

#[derive(Clone, serde::Serialize)]
#[allow(dead_code)]
struct TranscriptionPayload {
    text: String,
    is_final: bool,
    language: String,
}

fn main() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
            let show = MenuItemBuilder::with_id("show", "Show").build(app)?;
            let sep = tauri::menu::PredefinedMenuItem::separator(app)?;

            let menu = MenuBuilder::new(app)
                .item(&show)
                .item(&sep)
                .item(&quit)
                .build()?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .manage(AppState {
            is_recording: Mutex::new(false),
        })
        .invoke_handler(tauri::generate_handler![
            start_capture,
            stop_capture,
            get_capture_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
