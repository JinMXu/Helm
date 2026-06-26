#![windows_subsystem = "windows"]

//! Tauri GUI entry point for Helm.
//!
//! Wires helm-core to Tauri commands, starts a background scan loop that
//! broadcasts `helm://update` events every 2 seconds, and sets up the system
//! tray icon.

mod commands;
mod state;

use std::time::Duration;
use tauri::{Emitter, Manager};
use tauri::tray::{TrayIconBuilder, TrayIconEvent};
use tauri::menu::{Menu, MenuItem};

fn main() {
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(state::AppState::new())
        .setup(|app| {
            // System tray
            let show = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &quit])?;
            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .tooltip("Helm")
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click { button: tauri::tray::MouseButton::Left, .. } = event {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            // Background scan loop: emit ports://update every 2 seconds.
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let state = app_handle.state::<state::AppState>();
                loop {
                    let entries = state.scan_all();
                    let _ = app_handle.emit("helm://update", entries);
                    tokio::time::sleep(Duration::from_secs(2)).await;
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_ports,
            commands::port_info,
            commands::process_tree,
            commands::kill_port,
            commands::find_free_port,
            commands::open_in_browser,
            commands::get_git_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
