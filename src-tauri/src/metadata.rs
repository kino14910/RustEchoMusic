use std::path::Path;

use lofty::prelude::*;
use lofty::probe::Probe;
use lofty::tag::ItemKey;

use crate::models::Track;

pub fn parse_single_track(file_path: &Path) -> Option<Track> {
    let tagged_file = Probe::open(file_path).ok()?.read().ok()?;

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