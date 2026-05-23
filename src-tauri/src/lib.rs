mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        // .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::play_music,
            commands::toggle_music,
            commands::current_time,
            commands::set_current_time,
            commands::set_volume,
            commands::scan_music_directory,
            commands::load_music_library,
            commands::get_track_cover,
            commands::load_recently_played,
            commands::add_recently_played,
            commands::clear_recently_played,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
