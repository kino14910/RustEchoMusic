use std::collections::HashSet;
use std::fs::{self, File};
use std::path::Path;

use base64::{engine::general_purpose, Engine as _};
use lofty::prelude::*;
use lofty::probe::Probe;
use tauri::{command, Manager};
use walkdir::WalkDir;

use crate::commands::settings::load_settings;
use crate::metadata::parse_single_track;
use crate::models::settings::AppSettings;
use crate::models::Track;

const SUPPORTED_EXT: [&str; 5] = ["mp3", "flac", "m4a", "wav", "ogg"];

fn is_supported_audio_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| {
            SUPPORTED_EXT.contains(&ext.to_lowercase().as_str())
        })
        .unwrap_or(false)
}

fn scan_single_directory(dir: &str) -> Result<Vec<Track>, String> {
    let root_path = Path::new(dir);

    if !root_path.exists() || !root_path.is_dir() {
        return Err(format!("无效目录: {}", dir));
    }

    let mut tracks = Vec::new();

    for entry in WalkDir::new(root_path)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();

        if !path.is_file() || !is_supported_audio_file(path) {
            continue;
        }

        if let Some(mut track) = parse_single_track(path) {
            track.cover = None;
            tracks.push(track);
        }
    }

    Ok(tracks)
}

fn dedupe_tracks(tracks: &mut Vec<Track>) {
    let mut seen = HashSet::new();
    tracks.retain(|track| seen.insert(track.path.clone()));
}

pub fn execute_scan(dirs: Vec<String>) -> Result<Vec<Track>, String> {
    let mut all_tracks = Vec::new();

    for dir in dirs {
        if let Ok(mut tracks) = scan_single_directory(&dir) {
            all_tracks.append(&mut tracks);
        }
    }

    dedupe_tracks(&mut all_tracks);
    Ok(all_tracks)
}

pub fn save_music_library(
    app_handle: &tauri::AppHandle,
    library: &[Track],
) -> Result<(), String> {
    let app_data_path = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;

    if !app_data_path.exists() {
        fs::create_dir_all(&app_data_path)
            .map_err(|e| format!("创建数据目录失败: {}", e))?;
    }

    let json_file_path = app_data_path.join("library.json");
    let temp_file_path = app_data_path.join("library.json.tmp");

    let json_str = serde_json::to_string_pretty(library)
        .map_err(|e| e.to_string())?;

    fs::write(&temp_file_path, json_str).map_err(|e| e.to_string())?;
    fs::rename(temp_file_path, json_file_path).map_err(|e| e.to_string())?;

    Ok(())
}

#[command]
pub fn get_track_cover(full_path: String) -> Result<Option<String>, String> {
    let file_path = Path::new(&full_path);

    if !file_path.exists() {
        return Err("音频文件不存在".into());
    }

    let tagged_file = Probe::open(file_path)
        .map_err(|e| e.to_string())?
        .read()
        .map_err(|e| e.to_string())?;

    let tag = tagged_file
        .primary_tag()
        .or_else(|| tagged_file.first_tag());

    let cover = tag
        .and_then(|t| t.pictures().first())
        .map(|pic| {
            let b64_encoded = general_purpose::STANDARD.encode(pic.data());
            let mime_type = pic
                .mime_type()
                .map(|m| m.as_str())
                .unwrap_or("image/jpeg");

            format!("data:{};base64,{}", mime_type, b64_encoded)
        });

    Ok(cover)
}

#[command]
pub fn scan_music_directories(
    app_handle: tauri::AppHandle,
    dirs: Vec<String>,
) -> Result<Vec<Track>, String> {
    let tracks = execute_scan(dirs)?;
    save_music_library(&app_handle, &tracks)?;
    Ok(tracks)
}

#[command]
pub async fn load_music_library(
    app_handle: tauri::AppHandle,
) -> Result<Vec<Track>, String> {
    let app_data_path = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;

    let json_file_path = app_data_path.join("library.json");

    if !json_file_path.exists() {
        return rebuild_and_get_library(&app_handle).await;
    }

    let file = File::open(&json_file_path).map_err(|e| e.to_string())?;

    match serde_json::from_reader::<_, Vec<Track>>(file) {
        Ok(library) => Ok(library),
        Err(_) => {
            let _ = fs::remove_file(&json_file_path);
            rebuild_and_get_library(&app_handle).await
        }
    }
}

async fn rebuild_and_get_library(
    app_handle: &tauri::AppHandle,
) -> Result<Vec<Track>, String> {
    let settings: AppSettings = load_settings(app_handle.clone()).await?;

    let tracks = tauri::async_runtime::spawn_blocking({
        let handle = app_handle.clone();
        move || {
            let res = execute_scan(settings.library_dirs)?;
            save_music_library(&handle, &res)?;
            Ok::<Vec<Track>, String>(res)
        }
    })
    .await
    .map_err(|e| e.to_string())??;

    Ok(tracks)
}