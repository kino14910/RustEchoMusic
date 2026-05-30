mod audio;
mod commands;
mod metadata;
mod models;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager,
};

use commands::library::{
    execute_scan, get_track_cover, load_music_library, save_music_library, scan_music_directories,
};

use commands::playback::{current_time, play_music, set_current_time, set_volume, toggle_music};

use commands::recent::{add_recently_played, clear_recently_played, load_recently_played};

use commands::settings::{load_settings, save_settings};

pub fn init_startup_scan(app_handle: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        if let Ok(settings) = crate::commands::settings::load_settings(app_handle.clone()).await {
            if settings.scan_on_startup {
                if let Ok(tracks) = crate::commands::library::execute_scan(settings.library_dirs) {
                    let _ = crate::commands::library::save_music_library(&app_handle, &tracks);
                }
            }
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app: &tauri::AppHandle, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        ..
                    } => {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.unminimize();
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {
                        println!("unhandled event {event:?}");
                    }
                })
                .build(app)?;

            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Ok(settings) = load_settings(handle.clone()).await {
                    if settings.scan_on_startup {
                        let _ = tauri::async_runtime::spawn_blocking(move || {
                            if let Ok(tracks) = execute_scan(settings.library_dirs) {
                                if save_music_library(&handle, &tracks).is_ok() {
                                    let _ = handle.emit("library:refreshed", tracks);
                                }
                            }
                        })
                        .await;
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            play_music,
            toggle_music,
            current_time,
            set_current_time,
            set_volume,
            scan_music_directories,
            load_music_library,
            get_track_cover,
            load_recently_played,
            add_recently_played,
            clear_recently_played,
            save_settings,
            load_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
