use std::num::NonZeroUsize;
use std::path::Path;
use std::sync::Mutex;
use std::time::UNIX_EPOCH;

use lru::LruCache;

const CAPACITY: usize = 512;

pub struct CoverCache {
    inner: Mutex<LruCache<i64, (u64, Option<String>)>>,
}

impl CoverCache {
    pub fn new() -> Self {
        let capacity = NonZeroUsize::new(CAPACITY).unwrap_or(NonZeroUsize::MIN);
        Self {
            inner: Mutex::new(LruCache::new(capacity)),
        }
    }

    pub fn get(&self, track_id: i64, fingerprint: u64) -> Option<Option<String>> {
        let mut cache = self.inner.lock().ok()?;

        let hit = matches!(cache.get(&track_id), Some((cached, _)) if *cached == fingerprint);
        if !hit {
            cache.pop(&track_id);
            return None;
        }

        cache.get(&track_id).map(|(_, cover)| cover.clone())
    }

    pub fn insert(&self, track_id: i64, fingerprint: u64, cover: Option<String>) {
        if let Ok(mut cache) = self.inner.lock() {
            cache.put(track_id, (fingerprint, cover));
        }
    }

    pub fn invalidate(&self, track_id: i64) {
        if let Ok(mut cache) = self.inner.lock() {
            cache.pop(&track_id);
        }
    }

    pub fn clear(&self) {
        if let Ok(mut cache) = self.inner.lock() {
            cache.clear();
        }
    }
}

impl Default for CoverCache {
    fn default() -> Self {
        Self::new()
    }
}

pub fn file_fingerprint(path: &Path) -> Option<u64> {
    let meta = path.metadata().ok()?;

    let modified = meta
        .modified()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .unwrap_or(0);

    Some(modified.wrapping_shl(32) | (meta.len() & 0xffff_ffff))
}
