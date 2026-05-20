use std::fs::File;
use std::sync::OnceLock;

use tauri::command;

use web_audio_api::context::{AudioContext, BaseAudioContext};
use web_audio_api::node::{AudioNode, AudioScheduledSourceNode};
use web_audio_api::MediaElement;

fn get_audio_context() -> &'static AudioContext {
    static CONTEXT: OnceLock<AudioContext> = OnceLock::new();
    CONTEXT.get_or_init(|| AudioContext::default())
}

#[command]
pub async fn play_music(name: &str) -> Result<String, String> {
    let current_dir = std::env::current_dir().unwrap();
    let file_path = current_dir.join("..").join("static").join(name);

    let file = File::open(&file_path).map_err(|e| format!("Failed to open file: {}", e))?;

    let context = get_audio_context();

    let buffer = context
        .decode_audio_data(file)
        .await
        .map_err(|e| format!("Failed to decode audio data: {}", e))?;

    let mut src = context.create_buffer_source();
    src.set_buffer(buffer);
    src.set_loop(false);
    src.connect(&context.destination());

    src.start();

    Ok(format!("Playing music: {}", name))
}

#[command]
pub async fn pause_music() {
    let context = get_audio_context();
    context.suspend().await;
}

#[command]
pub async fn resume_music() {
    let context = get_audio_context();
    context.resume().await;
}

#[command]
pub async fn current_time() -> f64 {
    let context = get_audio_context();
    context.current_time()
}

#[command]
pub async fn set_current_time(time: f64) {
    let context = get_audio_context();
    // TODO: set_current_time
}