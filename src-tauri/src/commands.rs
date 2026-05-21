use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use lofty::prelude::*;
use lofty::probe::Probe;
use tauri::command;

use web_audio_api::context::{AudioContext, BaseAudioContext};
use web_audio_api::node::AudioNode;
use web_audio_api::MediaElement;

use serde::Serialize;

fn get_audio_context() -> &'static AudioContext {
    static CONTEXT: OnceLock<AudioContext> = OnceLock::new();
    CONTEXT.get_or_init(|| AudioContext::default())
}

struct AudioState {
    media: Option<MediaElement>,
}

fn get_audio_state() -> &'static Mutex<AudioState> {
    static STATE: OnceLock<Mutex<AudioState>> = OnceLock::new();
    STATE.get_or_init(|| {
        Mutex::new(AudioState { media: None })
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
    src.connect(&context.destination());

    media.set_loop(false);
    media.set_current_time(0.0);

    let mut state = get_audio_state().lock().map_err(|_| "Failed to lock audio state")?;

    if let Some(old_media) = state.media.replace(media) {
        old_media.pause();
    }

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
    let state = get_audio_state().lock().unwrap();
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
pub struct SongInfo {
    title: String,
    artist: String,
    album: String,
    duration: f64,
    sample_rate: Option<u32>
}

#[command]
pub fn get_song_info(name: String) -> SongInfo {
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

    let props = tagged_file.properties();
    let duration = props.duration().as_secs_f64();

    SongInfo {
        title,
        artist,
        album,
        duration,
        sample_rate: props.sample_rate(),
    }
}
