mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::play_music,
            commands::pause_music,
            commands::resume_music,
            commands::current_time,
            commands::set_current_time
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
