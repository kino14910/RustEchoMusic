import { invoke } from '@tauri-apps/api/core'
import type { Track } from '../types/music'

class MusicLibrary {
    tracks = $state<Track[]>([])
    isLoading = $state(false)
    error = $state<string | null>(null)

    private refreshPromise: Promise<Track[]> | null = null

    async refresh(options: { force?: boolean } = {}): Promise<Track[]> {
        const { force = false } = options

        if (this.refreshPromise) {
            return this.refreshPromise
        }

        if (!force && this.tracks.length > 0) {
            return this.tracks
        }

        this.isLoading = true
        this.error = null

        this.refreshPromise = (async () => {
            try {
                const tracks = await invoke<Track[]>('load_music_library')
                this.tracks = tracks
                return tracks
            } catch (err) {
                console.error('全局加载媒体库失败:', err)
                this.error = String(err)
                return this.tracks
            } finally {
                this.isLoading = false
                this.refreshPromise = null
            }
        })()

        return this.refreshPromise
    }
}

export const musicLibrary = new MusicLibrary()