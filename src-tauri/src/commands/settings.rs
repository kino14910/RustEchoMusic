use std::fs;
use std::fs::File;
use std::path::PathBuf;

use tauri::{command, Manager};

use crate::models::settings::AppSettings;

const SETTINGS_FILE: &str = "settings.json";

fn settings_path(app_handle: &tauri::AppHandle) -> Result<PathBuf, String> {
    let app_data_path = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("无法获取应用数据目录: {}", e))?;

    if !app_data_path.exists() {
        fs::create_dir_all(&app_data_path)
            .map_err(|e| format!("创建应用数据目录失败: {}", e))?;
    }

    Ok(app_data_path.join(SETTINGS_FILE))
}

#[command]
pub async fn load_settings(app_handle: tauri::AppHandle) -> Result<AppSettings, String> {
    let path = settings_path(&app_handle)?;

    if !path.exists() {
        let settings = AppSettings::default();
        save_settings(app_handle, settings.clone()).await?;
        return Ok(settings);
    }

    let file = File::open(&path).map_err(|e| format!("无法打开设置文件: {}", e))?;

    match serde_json::from_reader::<_, AppSettings>(file) {
        Ok(settings) => Ok(settings),
        Err(_) => {
            let settings = AppSettings::default();
            save_settings(app_handle, settings.clone()).await?;
            Ok(settings)
        }
    }
}

#[command]
pub async fn save_settings(
    app_handle: tauri::AppHandle,
    settings: AppSettings,
) -> Result<AppSettings, String> {
    let path = settings_path(&app_handle)?;
    let temp_path = path.with_extension("json.tmp");

    let json = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("序列化设置失败: {}", e))?;

    fs::write(&temp_path, json).map_err(|e| format!("写入设置临时文件失败: {}", e))?;

    fs::rename(temp_path, path).map_err(|e| format!("保存设置失败: {}", e))?;

    Ok(settings)
}