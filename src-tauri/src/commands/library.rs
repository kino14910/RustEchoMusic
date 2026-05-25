use std::fs::{self, File};
use std::path::Path;

use base64::{engine::general_purpose, Engine as _};
use lofty::prelude::*;
use lofty::probe::Probe;
use tauri::{command, Manager};
use walkdir::WalkDir;

use crate::metadata::parse_single_track;
use crate::models::Track;

const SUPPORTED_EXT: [&str; 5] = ["mp3", "flac", "m4a", "wav", "ogg"];

#[command]
pub async fn get_track_cover(full_path: String) -> Result<Option<String>, String> {
    let file_path = Path::new(&full_path);

    if !file_path.exists() {
        return Err("音频文件不存在或已被移动".into());
    }

    let tagged_file = Probe::open(file_path)
        .map_err(|e| format!("无法打开音频文件: {}", e))?
        .read()
        .map_err(|e| format!("无法读取音频元数据: {}", e))?;

    let tag = tagged_file
        .primary_tag()
        .or_else(|| tagged_file.first_tag());

    let cover = tag.and_then(|t| t.pictures().first()).map(|pic| {
        let b64_encoded = general_purpose::STANDARD.encode(pic.data());
        let mime_type = pic.mime_type().map(|m| m.as_str()).unwrap_or("image/jpeg");

        format!("data:{};base64,{}", mime_type, b64_encoded)
    });

    Ok(cover)
}

async fn scan_music_directory_logic(
    app_handle: &tauri::AppHandle,
) -> Result<Vec<Track>, String> {
    let music_dir = app_handle
        .path()
        .audio_dir()
        .map_err(|e| format!("无法获取系统音乐目录: {}", e))?;

    let mut tracks = Vec::new();

    visit_dirs(&music_dir, &mut tracks);

    Ok(tracks)
}

fn visit_dirs(dir: &Path, tracks: &mut Vec<Track>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_dir() {
                visit_dirs(&path, tracks);
                continue;
            }

            if !is_supported_audio_file(&path) {
                continue;
            }

            if let Some(track) = parse_single_track(&path) {
                tracks.push(track);
            }
        }
    }
}

fn is_supported_audio_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| SUPPORTED_EXT.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

#[command]
pub async fn scan_music_directory(
    app_handle: tauri::AppHandle,
    dir_path: String,
) -> Result<Vec<Track>, String> {
    let root_path = Path::new(&dir_path);

    if !root_path.is_dir() {
        return Err("Invalid directory path".into());
    }

    let mut playlist = Vec::new();

    for entry in WalkDir::new(root_path).into_iter().filter_map(|entry| entry.ok()) {
        let path = entry.path();

        if !path.is_file() || !is_supported_audio_file(path) {
            continue;
        }

        if let Some(track) = parse_single_track(path) {
            playlist.push(track);
        }
    }

    save_music_library(&app_handle, &playlist)?;

    Ok(playlist)
}

#[command]
pub async fn load_music_library(
    app_handle: tauri::AppHandle,
) -> Result<Vec<Track>, String> {
    let app_data_path = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("无法获取数据目录: {}", e))?;

    if !app_data_path.exists() {
        fs::create_dir_all(&app_data_path)
            .map_err(|e| format!("创建数据目录失败: {}", e))?;
    }

    let json_file_path = app_data_path.join("library.json");

    if !json_file_path.exists() {
        return rebuild_and_get_library(&app_handle).await;
    }

    let file = File::open(&json_file_path)
        .map_err(|e| format!("无法打开库文件: {}", e))?;

    match serde_json::from_reader::<_, Vec<Track>>(file) {
        Ok(mut library) => {
            let had_covers = library.iter().any(|track| track.cover.is_some());

            if had_covers {
                for track in &mut library {
                    track.cover = None;
                }

                save_music_library(&app_handle, &library)?;
            }

            Ok(library)
        }
        Err(error) => {
            println!("[load_music_library] JSON 解析失败，准备重建: {}", error);

            let _ = fs::remove_file(json_file_path);

            rebuild_and_get_library(&app_handle).await
        }
    }
}

async fn rebuild_and_get_library(
    app_handle: &tauri::AppHandle,
) -> Result<Vec<Track>, String> {
    let fresh_library = scan_music_directory_logic(app_handle).await?;

    save_music_library(app_handle, &fresh_library)?;

    Ok(fresh_library)
}

pub fn save_music_library(
    app_handle: &tauri::AppHandle,
    library: &[Track],
) -> Result<(), String> {
    let app_data_path = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("{}", e))?;

    if !app_data_path.exists() {
        fs::create_dir_all(&app_data_path)
            .map_err(|e| format!("创建数据目录失败: {}", e))?;
    }

    let json_file_path = app_data_path.join("library.json");
    let temp_file_path = app_data_path.join("library.json.tmp");

    let json_str = serde_json::to_string_pretty(library)
        .map_err(|e| format!("{}", e))?;

    fs::write(&temp_file_path, json_str)
        .map_err(|e| format!("{}", e))?;

    fs::rename(temp_file_path, json_file_path)
        .map_err(|e| format!("{}", e))?;

    Ok(())
}