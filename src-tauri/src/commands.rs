use base64::{engine::general_purpose, Engine as _};
use lofty::prelude::*;
use lofty::probe::Probe;
use std::fs::{self, File};
use std::path::Path;
use std::sync::{Mutex, OnceLock};
use tauri::{command, Manager};

use walkdir::WalkDir;
use web_audio_api::context::{AudioContext, BaseAudioContext};
use web_audio_api::node::{AudioNode, GainNode};
use web_audio_api::MediaElement;

use serde::{Deserialize, Serialize};

const RECENTLY_PLAYED_FILE: &str = "recently_played.json";
const MAX_RECENTLY_PLAYED: usize = 100;

fn get_audio_context() -> &'static AudioContext {
    static CONTEXT: OnceLock<AudioContext> = OnceLock::new();
    CONTEXT.get_or_init(|| AudioContext::default())
}

struct AudioState {
    media: Option<MediaElement>,
    gain_node: Option<GainNode>,
    volume: f32,
}

fn get_audio_state() -> &'static Mutex<AudioState> {
    static STATE: OnceLock<Mutex<AudioState>> = OnceLock::new();
    STATE.get_or_init(|| {
        Mutex::new(AudioState {
            media: None,
            gain_node: None,
            volume: 0.8,
        })
    })
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Track {
    title: String,
    artist: String,
    album: String,
    album_artist: String,
    duration: f64,
    sample_rate: Option<u32>,
    cover: Option<String>,
    path: String,
}

#[command]
pub async fn play_music(full_path: &str) -> Result<String, String> {
    let file_path = Path::new(&full_path);
    if !file_path.exists() {
        return Err("音频文件不存在或已被移动".into());
    }

    let mut media = MediaElement::new(&file_path)
        .map_err(|e| format!("Failed to create media element: {}", e))?;
    let context = get_audio_context();
    let src = context.create_media_element_source(&mut media);

    // src.connect(&context.destination());
    let gain_node = context.create_gain();

    src.connect(&gain_node);
    gain_node.connect(&context.destination());

    media.set_loop(false);
    media.set_current_time(0.0);

    let mut state = get_audio_state()
        .lock()
        .map_err(|_| "Failed to lock audio state")?;

    gain_node.gain().set_value(state.volume);

    if let Some(old_media) = state.media.replace(media) {
        old_media.pause();
    }

    state.gain_node = Some(gain_node);

    if let Some(ref media) = state.media {
        media.play();
    }

    Ok(format!("Playing music: {}", full_path))
}

#[command]
pub async fn toggle_music() -> Result<bool, String> {
    let state = get_audio_state()
        .lock()
        .map_err(|_| "Failed to lock audio state")?;
    if let Some(ref media) = state.media {
        if media.paused() {
            media.play();
            Ok(true)
        } else {
            media.pause();
            Ok(false)
        }
    } else {
        Err("No media available".into())
    }
}

#[command]
pub async fn current_time() -> f64 {
    let Ok(state) = get_audio_state().lock() else {
        return 0.0;
    };
    if let Some(ref media) = state.media {
        media.current_time()
    } else {
        0.0
    }
}

#[command]
pub async fn set_current_time(time: f64) {
    let state = get_audio_state().lock().unwrap();
    if let Some(ref media) = state.media {
        media.set_current_time(time);
    }
}

#[command]
pub async fn set_volume(volume: u8) -> Result<(), String> {
    let mut state = get_audio_state()
        .lock()
        .map_err(|_| "Failed to lock audio state")?;
    let volume: f32 = volume as f32 / 100.0;
    let safe_volume = volume.clamp(0.0, 1.0);
    state.volume = safe_volume;

    if let Some(ref gain_node) = state.gain_node {
        gain_node.gain().set_value(safe_volume);
    }

    Ok(())
}

fn parse_single_track(file_path: &Path) -> Option<Track> {
    let tagged_file = Probe::open(file_path).ok()?.read().ok()?;
    for tag in tagged_file.tags() {
        println!("==== TAG {:?} ====", tag.tag_type());

        for item in tag.items() {
            println!("{:?} = {:?}", item.key(), item.value());
        }
    }
    let tag = tagged_file
        .primary_tag()
        .or_else(|| tagged_file.first_tag());

    let title = tag
        .and_then(|t| t.title())
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            file_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Unknown Track")
                .to_string()
        });

    let artist = tag
        .and_then(|t| t.artist())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "Unknown Artist".to_string());

    let album = tag
        .and_then(|t| t.album())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "Unknown Album".to_string());

    let album_artist = tag
        .and_then(|t| {
            t.get_string(ItemKey::AlbumArtist)
                .or_else(|| t.get_string(ItemKey::AlbumArtists))
        })
        .map(|s| s.to_string())
        .unwrap_or_default();

    let props = tagged_file.properties();
    let duration = props.duration().as_secs_f64();
    let sample_rate = props.sample_rate();

    Some(Track {
        title,
        artist,
        album,
        album_artist,
        duration,
        sample_rate,
        cover: None,
        path: file_path.to_string_lossy().into_owned(),
    })
}

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

async fn scan_music_directory_logic(app_handle: &tauri::AppHandle) -> Result<Vec<Track>, String> {
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
            } else if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                let ext_lower = ext.to_lowercase();
                if ext_lower == "mp3"
                    || ext_lower == "flac"
                    || ext_lower == "wav"
                    || ext_lower == "m4a"
                    || ext_lower == "ogg"
                {
                    if let Some(track) = parse_single_track(&path) {
                        tracks.push(track);
                    }
                }
            }
        }
    }
}

static SUPPORTED_EXT: [&str; 4] = ["mp3", "flac", "m4a", "wav"];

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

    for entry in WalkDir::new(root_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            let ext = ext.to_lowercase();
            if SUPPORTED_EXT.contains(&ext.as_str()) {
                if let Some(track) = parse_single_track(path) {
                    playlist.push(track);
                }
            }
        }
    }

    let app_data_path = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("无法获取应用数据目录: {}", e))?;

    if !app_data_path.exists() {
        fs::create_dir_all(&app_data_path).map_err(|e| format!("创建数据目录失败: {}", e))?;
    }

    let json_file_path = app_data_path.join("library.json");

    let file = File::create(&json_file_path).map_err(|e| format!("无法创建库文件: {}", e))?;

    serde_json::to_writer_pretty(file, &playlist).map_err(|e| format!("写入 JSON 失败: {}", e))?;

    Ok(playlist)
}

#[command]
pub async fn load_music_library(app_handle: tauri::AppHandle) -> Result<Vec<Track>, String> {
    let app_data_path = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("无法获取数据目录: {}", e))?;

    if !app_data_path.exists() {
        fs::create_dir_all(&app_data_path).map_err(|e| format!("创建数据目录失败: {}", e))?;
    }

    let json_file_path = app_data_path.join("library.json");

    if !json_file_path.exists() {
        return rebuild_and_get_library(&app_handle).await;
    }

    let file = File::open(&json_file_path).map_err(|e| format!("无法打开库文件: {}", e))?;

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
        Err(e) => {
            println!("[load_music_library] JSON 解析失败，准备重建: {}", e);
            let _ = std::fs::remove_file(json_file_path);
            rebuild_and_get_library(&app_handle).await
        }
    }
}

async fn rebuild_and_get_library(app_handle: &tauri::AppHandle) -> Result<Vec<Track>, String> {
    let fresh_library = scan_music_directory_logic(app_handle).await?;

    save_music_library(app_handle, &fresh_library)?;

    Ok(fresh_library)
}

pub fn save_music_library(
    app_handle: &tauri::AppHandle,
    library: &Vec<Track>,
) -> Result<(), String> {
    let app_data_path = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("{}", e))?;

    let json_file_path = app_data_path.join("library.json");
    let temp_file_path = app_data_path.join("library.json.tmp");

    let json_str = serde_json::to_string_pretty(library).map_err(|e| format!("{}", e))?;

    std::fs::write(&temp_file_path, json_str).map_err(|e| format!("{}", e))?;
    std::fs::rename(temp_file_path, json_file_path).map_err(|e| format!("{}", e))?;

    Ok(())
}

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

fn read_recently_played(app_handle: &tauri::AppHandle) -> Result<Vec<Track>, String> {
    let file_path = recently_played_path(app_handle)?;

    if !file_path.exists() {
        return Ok(Vec::new());
    }

    let file = File::open(&file_path).map_err(|e| format!("无法打开最近播放文件: {}", e))?;

    serde_json::from_reader::<_, Vec<Track>>(file)
        .map_err(|e| format!("无法解析最近播放文件: {}", e))
}

fn save_recently_played(app_handle: &tauri::AppHandle, tracks: &[Track]) -> Result<(), String> {
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

    // 不建议把 base64 封面写入 recently_played.json，否则文件会越来越大
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
