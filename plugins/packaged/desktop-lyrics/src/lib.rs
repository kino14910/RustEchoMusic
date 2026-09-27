use plugin_sdk::guest::{GuestPlugin, Host};
use plugin_sdk::guest_plugin;
use plugin_sdk::{PluginError, PluginResult};
use serde_json::{json, Value};
use std::sync::Mutex;

const WINDOW_ROUTE: &str = "/desktop-lyrics";
const WINDOW_WIDTH: f64 = 960.0;
const WINDOW_HEIGHT: f64 = 240.0;

const SERVICE_WINDOW: &str = "desktop.window";
const SERVICE_PLAYER_STATE: &str = "player.state";
const SERVICE_PLAYER_CONTROL: &str = "player.control";

struct LyricLine {
    timestamp_ms: i64,
    text: String,
}

#[derive(Default)]
struct State {
    song_id: Option<i64>,
    lines: Vec<LyricLine>,
    visible: bool,
}

#[derive(Default)]
struct DesktopLyrics {
    state: Mutex<State>,
}

impl DesktopLyrics {
    fn call_service(host: &Host, service: &str, method: &str, args: &Value) -> PluginResult<Value> {
        let handle = host.resolve(service)?;
        let result = host.call(handle, method, args);
        host.release(handle);
        result
    }

    fn open_window(&self, host: &Host) -> PluginResult<()> {
        Self::call_service(
            host,
            SERVICE_WINDOW,
            "open",
            &json!({
                "route": WINDOW_ROUTE,
                "width": WINDOW_WIDTH,
                "height": WINDOW_HEIGHT,
            }),
        )?;
        self.set_visible(true);
        Ok(())
    }

    fn close_window(&self, host: &Host) -> PluginResult<()> {
        Self::call_service(host, SERVICE_WINDOW, "close", &Value::Null)?;
        self.set_visible(false);
        Ok(())
    }

    fn set_visible(&self, visible: bool) {
        if let Ok(mut state) = self.state.lock() {
            state.visible = visible;
        }
    }

    fn is_visible(&self) -> bool {
        self.state.lock().map(|state| state.visible).unwrap_or(false)
    }

    fn toggle(&self, host: &Host) -> PluginResult<Value> {
        if self.is_visible() {
            self.close_window(host)?;
            Ok(json!({ "visible": false }))
        } else {
            self.open_window(host)?;
            Ok(json!({ "visible": true }))
        }
    }

    fn control(&self, host: &Host, method: &str) -> PluginResult<Value> {
        // host.log(2, &format!("desktop-lyrics: {method}"));
        Self::call_service(host, SERVICE_PLAYER_CONTROL, method, &Value::Null)?;
        Ok(Value::Null)
    }

    fn toggle_playback(&self, host: &Host) -> PluginResult<Value> {
        // host.log(2, "desktop-lyrics: playPause");
        let handle = host.resolve(SERVICE_PLAYER_STATE)?;
        let playing = host.call(handle, "isPlaying", &Value::Null);
        host.release(handle);

        let playing = playing?.as_bool().unwrap_or(false);
        self.control(host, if playing { "pause" } else { "play" })?;
        Ok(json!({ "playing": !playing }))
    }

    fn snapshot(&self, host: &Host) -> PluginResult<Value> {
        let handle = host.resolve(SERVICE_PLAYER_STATE)?;
        let track = host.call(handle, "currentTrackId", &Value::Null);
        let position = host.call(handle, "currentTimeSecs", &Value::Null);
        let playing = host.call(handle, "isPlaying", &Value::Null);
        host.release(handle);

        let track_id = track?.as_i64();
        let position_ms = (position?.as_f64().unwrap_or(0.0) * 1000.0).round() as i64;
        let playing = playing?.as_bool().unwrap_or(false);

        let state = self
            .state
            .lock()
            .map_err(|error| PluginError::plugin(error.to_string()))?;

        let active_index = state
            .lines
            .iter()
            .enumerate()
            .take_while(|(_, line)| line.timestamp_ms <= position_ms)
            .map(|(index, _)| index)
            .last();

        let lines = state
            .lines
            .iter()
            .map(|line| json!({ "timestampMs": line.timestamp_ms, "text": line.text }))
            .collect::<Vec<_>>();

        Ok(json!({
            "visible": state.visible,
            "songId": state.song_id,
            "trackId": track_id,
            "playing": playing,
            "positionMs": position_ms,
            "activeIndex": active_index,
            "lines": lines,
        }))
    }

    fn replace_lines(&self, song_id: Option<i64>, lines: Vec<LyricLine>) {
        if let Ok(mut state) = self.state.lock() {
            state.song_id = song_id;
            state.lines = lines;
        }
    }
}

impl GuestPlugin for DesktopLyrics {
    fn descriptor(&self) -> Value {
        json!({
            "id": "desktop-lyrics",
            "version": "0.1.0",
            "minHost": "1.0.0",
            "abi": plugin_sdk::ABI_VERSION,
            "displayName": "Desktop Lyrics",
            "summary": "桌面悬浮歌词：跟随播放进度高亮当前歌词行",
            "capabilities": ["player.read", "player.control", "desktop.window"],
            "dependsOn": ["lyrics"],
            "optionalDependsOn": []
        })
    }

    fn activate(&self, _host: &Host) -> PluginResult<Value> {
        Ok(json!({
            "contributions": [
                {
                    "kind": "command",
                    "id": "desktop-lyrics.show",
                    "title": "显示桌面歌词",
                    "category": null,
                    "inputSchema": null
                },
                {
                    "kind": "command",
                    "id": "desktop-lyrics.hide",
                    "title": "隐藏桌面歌词",
                    "category": null,
                    "inputSchema": null
                },
                {
                    "kind": "command",
                    "id": "desktop-lyrics.toggle",
                    "title": "切换桌面歌词",
                    "category": null,
                    "inputSchema": null
                },
                {
                    "kind": "command",
                    "id": "desktop-lyrics.snapshot",
                    "title": "桌面歌词快照",
                    "category": null,
                    "inputSchema": null
                },
                {
                    "kind": "command",
                    "id": "desktop-lyrics.playPause",
                    "title": "播放 / 暂停",
                    "category": null,
                    "inputSchema": null
                },
                {
                    "kind": "command",
                    "id": "desktop-lyrics.next",
                    "title": "下一首",
                    "category": null,
                    "inputSchema": null
                },
                {
                    "kind": "command",
                    "id": "desktop-lyrics.previous",
                    "title": "上一首",
                    "category": null,
                    "inputSchema": null
                },
                {
                    "kind": "extension",
                    "point": "ui.playerAction",
                    "payload": {
                        "id": "desktop-lyrics",
                        "icon": "lyrics",
                        "title": "桌面歌词",
                        "command": "desktop-lyrics.toggle",
                        "order": 20
                    }
                }
            ],
            "subscriptions": [
                { "kinds": ["lyrics.loaded"] },
                { "kinds": ["track.changed"] }
            ],
            "serviceIds": []
        }))
    }

    fn command(&self, host: &Host, command: &str, _args: &Value) -> PluginResult<Value> {
        match command {
            "desktop-lyrics.show" => {
                self.open_window(host)?;
                Ok(json!({ "visible": true }))
            }
            "desktop-lyrics.hide" => {
                self.close_window(host)?;
                Ok(json!({ "visible": false }))
            }
            "desktop-lyrics.toggle" => self.toggle(host),
            "desktop-lyrics.snapshot" => self.snapshot(host),
            "desktop-lyrics.playPause" => self.toggle_playback(host),
            "desktop-lyrics.next" => self.control(host, "next"),
            "desktop-lyrics.previous" => self.control(host, "previous"),
            other => Err(PluginError::not_found(format!(
                "desktop-lyrics 无命令 '{other}'"
            ))),
        }
    }

    fn on_event(&self, _host: &Host, event: &Value) -> PluginResult<()> {
        let kind = match event.get("kind").and_then(Value::as_str) {
            Some(kind) => kind,
            None => return Ok(()),
        };
        let payload = event.get("payload").cloned().unwrap_or(Value::Null);

        match kind {
            "lyrics.loaded" => {
                let song_id = payload.get("songId").and_then(Value::as_i64);
                let lines = payload
                    .get("lines")
                    .and_then(Value::as_array)
                    .map(|entries| {
                        entries
                            .iter()
                            .filter_map(|entry| {
                                Some(LyricLine {
                                    timestamp_ms: entry.get("timestampMs")?.as_i64()?,
                                    text: entry.get("text")?.as_str()?.to_string(),
                                })
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                self.replace_lines(song_id, lines);
            }
            "track.changed" => {
                let track_id = payload.get("trackId").and_then(Value::as_i64);
                self.replace_lines(track_id, Vec::new());
            }
            _ => {}
        }
        Ok(())
    }

    fn deactivate(&self) -> PluginResult<()> {
        self.replace_lines(None, Vec::new());
        self.set_visible(false);
        Ok(())
    }
}

guest_plugin!(DesktopLyrics);
