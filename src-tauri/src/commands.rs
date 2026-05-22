use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use base64::{Engine as _, engine::general_purpose};
use lofty::prelude::*;
use lofty::probe::Probe;
use tauri::command;

use web_audio_api::context::{AudioContext, BaseAudioContext};
use web_audio_api::node::{AudioNode, GainNode};
use web_audio_api::MediaElement;

use serde::Serialize;

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
            volume: 0.8 })
    })
}

fn test_path() -> PathBuf {
    let current_dir = std::env::current_dir().unwrap();
    current_dir.join("..").join("static")
}

#[command]
pub async fn play_music(name: &str) -> Result<String, String> {
    let file_path = test_path().join(name);

    let mut media = MediaElement::new(&file_path).map_err(|e| format!("Failed to create media element: {}", e))?;
    let context = get_audio_context();
    let src = context.create_media_element_source(&mut media);
    
    // src.connect(&context.destination());
    let gain_node = context.create_gain();

    src.connect(&gain_node);
    gain_node.connect(&context.destination());

    media.set_loop(false);
    media.set_current_time(0.0);

    let mut state = get_audio_state().lock().map_err(|_| "Failed to lock audio state")?;
    
    gain_node.gain().set_value(state.volume);

    if let Some(old_media) = state.media.replace(media) {
        old_media.pause();
    }

    state.gain_node = Some(gain_node);

    if let Some(ref media) = state.media {
        media.play();
    }

    Ok(format!("Playing music: {}", name))
}

#[command]
pub async fn toggle_music() -> Result<bool, String> {
    let state = get_audio_state().lock().map_err(|_| "Failed to lock audio state")?;
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


#[derive(Serialize)]
pub struct TrackInfo {
    title: String,
    artist: String,
    album: String,
    duration: f64,
    sample_rate: Option<u32>,
    cover: Option<String>,
}

#[command]
pub fn get_track_info(name: String) -> TrackInfo {
    let file_path = test_path().join(name);
    let tagged_file = Probe::open(&file_path)
        .unwrap()
        .read()
        .unwrap();

    let tag = tagged_file.primary_tag()
        .or_else(|| tagged_file.first_tag());

    let title = tag
        .and_then(|t| t.title())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "Unknown".to_string());
    let artist = tag
        .and_then(|t| t.artist())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "Unknown".to_string());
    let album = tag
        .and_then(|t| t.album())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let cover = tag.and_then(|t| t.pictures().first()).map(|pic| {
        let b64_encoded = general_purpose::STANDARD.encode(pic.data());
        let mime_type = pic.mime_type()
            .map(|m| m.as_str())
            .unwrap_or("image/jpeg");
        format!("data:{};base64,{}", mime_type, b64_encoded)
    });

    let props = tagged_file.properties();
    let duration = props.duration().as_secs_f64();

    TrackInfo {
        title,
        artist,
        album,
        duration,
        sample_rate: props.sample_rate(),
        cover,
    }
}

#[command]
pub async fn set_volume(volume: u8) -> Result<(), String> {
    let mut state = get_audio_state().lock().map_err(|_| "Failed to lock audio state")?;
    let volume: f32 = volume as f32 / 100.0;
    let safe_volume = volume.clamp(0.0, 1.0);
    state.volume = safe_volume;

    if let Some(ref gain_node) = state.gain_node {
        gain_node.gain().set_value(safe_volume);
    }

    Ok(())
}
