use std::path::Path;

use tauri::command;
use web_audio_api::context::BaseAudioContext;
use web_audio_api::node::AudioNode;
use web_audio_api::MediaElement;

use crate::audio::{get_audio_context, get_audio_state};

#[command]
pub async fn play_music(full_path: &str) -> Result<String, String> {
    let file_path = Path::new(full_path);

    if !file_path.exists() {
        return Err("音频文件不存在或已被移动".into());
    }

    let mut media = MediaElement::new(file_path)
        .map_err(|e| format!("Failed to create media element: {}", e))?;

    let context = get_audio_context();
    let src = context.create_media_element_source(&mut media);

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
pub async fn resume_music() -> Result<(), String> {
    let state = get_audio_state()
        .lock()
        .map_err(|_| "Failed to lock audio state")?;

    if let Some(ref media) = state.media {
        media.play();
        Ok(())
    } else {
        Err("No media available".into())
    }
}

#[command]
pub async fn pause_music() -> Result<(), String> {
    let state = get_audio_state()
        .lock()
        .map_err(|_| "Failed to lock audio state")?;

    if let Some(ref media) = state.media {
        media.pause();
        Ok(())
    } else {
        Err("No media available".into())
    }
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
    let Ok(state) = get_audio_state().lock() else {
        return;
    };

    if let Some(ref media) = state.media {
        media.set_current_time(time);
    }
}

#[command]
pub async fn set_volume(volume: u8) -> Result<(), String> {
    let mut state = get_audio_state()
        .lock()
        .map_err(|_| "Failed to lock audio state")?;

    let volume = volume as f32 / 100.0;
    let safe_volume = volume.clamp(0.0, 1.0);

    state.volume = safe_volume;

    if let Some(ref gain_node) = state.gain_node {
        gain_node.gain().set_value(safe_volume);
    }

    Ok(())
}