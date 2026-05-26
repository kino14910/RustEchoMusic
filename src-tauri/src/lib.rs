mod audio;
mod commands;
mod metadata;
mod models;

use std::thread;

use commands::library::{
    get_track_cover,
    load_music_library,
    scan_music_directory,
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

fn scan_media_libraries(dirs: Vec<String>) {
    println!("开始扫描以下目录中的音乐文件: {:?}", dirs);
    // 扫描
    
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        // .plugin(tauri_plugin_opener::init())
        // .setup(|app| {
        //     let app_handle = app.handle().clone();
            
        //     thread::spawn(move || {
        //         let scan_on_startup = true;
        //         let library_dirs = vec![String::from("D:/Music")];

        //         if scan_on_startup && !library_dirs.is_empty() {
        //             scan_media_libraries(library_dirs);
        //             // app_handle.emit("scan-finished", "success").unwrap();
        //         }
        //     });

        //     Ok(())
        // })
        .invoke_handler(tauri::generate_handler![
            play_music,
            toggle_music,
            current_time,
            set_current_time,
            set_volume,
            scan_music_directory,
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
