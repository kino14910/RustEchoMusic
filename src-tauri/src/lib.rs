mod audio;
mod commands;
mod metadata;
mod models;

use commands::library::{
    get_track_cover,
    save_music_library,
    load_music_library,
    execute_scan,
    scan_music_directories,
};

use commands::playback::{
    current_time,
    play_music,
    set_current_time,
    set_volume,
    toggle_music,
};

use commands::recent::{
    add_recently_played,
    clear_recently_played,
    load_recently_played,
};

use commands::settings::{
    load_settings,
    save_settings,
};
use tauri::Emitter;

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
        .setup(|app| {
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
