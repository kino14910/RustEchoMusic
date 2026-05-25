use std::sync::{Mutex, OnceLock};

use web_audio_api::context::AudioContext;
use web_audio_api::node::GainNode;
use web_audio_api::MediaElement;

pub fn get_audio_context() -> &'static AudioContext {
    static CONTEXT: OnceLock<AudioContext> = OnceLock::new();
    CONTEXT.get_or_init(|| AudioContext::default())
}

pub struct AudioState {
    pub media: Option<MediaElement>,
    pub gain_node: Option<GainNode>,
    pub volume: f32,
}

pub fn get_audio_state() -> &'static Mutex<AudioState> {
    static STATE: OnceLock<Mutex<AudioState>> = OnceLock::new();

    STATE.get_or_init(|| {
        Mutex::new(AudioState {
            media: None,
            gain_node: None,
            volume: 0.8,
        })
    })
}