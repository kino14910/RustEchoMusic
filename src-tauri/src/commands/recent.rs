use crate::models::Track;

use std::fs::{self, File};
use tauri::{command, Manager};

const RECENTLY_PLAYED_FILE: &str = "recently_played.json";
const MAX_RECENTLY_PLAYED: usize = 100;

fn recently_played_path(app_handle: &tauri::AppHandle) -> Result<std::path::PathBuf, String> {
    let app_data_path = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("无法获取应用数据目录: {}", e))?;

    if !app_data_path.exists() {
        fs::create_dir_all(&app_data_path).map_err(|e| format!("创建数据目录失败: {}", e))?;
    }

    Ok(app_data_path.join(RECENTLY_PLAYED_FILE))
}

pub fn read_recently_played(app_handle: &tauri::AppHandle) -> Result<Vec<Track>, String> {
    let file_path = recently_played_path(app_handle)?;

    if !file_path.exists() {
        return Ok(Vec::new());
    }

    let file = File::open(&file_path).map_err(|e| format!("无法打开最近播放文件: {}", e))?;

    serde_json::from_reader::<_, Vec<Track>>(file)
        .map_err(|e| format!("无法解析最近播放文件: {}", e))
}

pub fn save_recently_played(app_handle: &tauri::AppHandle, tracks: &[Track]) -> Result<(), String> {
    let file_path = recently_played_path(app_handle)?;
    let temp_file_path = file_path.with_extension("json.tmp");

    let json =
        serde_json::to_string_pretty(tracks).map_err(|e| format!("序列化最近播放失败: {}", e))?;

    std::fs::write(&temp_file_path, json)
        .map_err(|e| format!("写入最近播放临时文件失败: {}", e))?;

    std::fs::rename(temp_file_path, file_path).map_err(|e| format!("保存最近播放失败: {}", e))?;

    Ok(())
}

#[command]
pub async fn load_recently_played(app_handle: tauri::AppHandle) -> Result<Vec<Track>, String> {
    read_recently_played(&app_handle)
}

#[command]
pub async fn add_recently_played(
    app_handle: tauri::AppHandle,
    track: Track,
) -> Result<Vec<Track>, String> {
    let mut list = read_recently_played(&app_handle).unwrap_or_default();

    let mut track = track;

    // 不要把 base64 封面写入 recently_played.json，否则文件会越来越大
    track.cover = None;

    // 去重：同一路径只保留一条，并把新的放到最前面
    list.retain(|item| item.path != track.path);
    list.insert(0, track);

    if list.len() > MAX_RECENTLY_PLAYED {
        list.truncate(MAX_RECENTLY_PLAYED);
    }

    save_recently_played(&app_handle, &list)?;

    Ok(list)
}

#[command]
pub async fn clear_recently_played(app_handle: tauri::AppHandle) -> Result<(), String> {
    save_recently_played(&app_handle, &[])?;
    Ok(())
}
