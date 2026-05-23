import { invoke } from '@tauri-apps/api/core'
import type { TrackInfo } from './player.svelte'

class MusicLibrary {
    tracks = $state<TrackInfo[]>([])
    isLoading = $state(false)
    error = $state<string | null>(null)

    private refreshPromise: Promise<TrackInfo[]> | null = null

    async refresh(options: { force?: boolean } = {}): Promise<TrackInfo[]> {
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
                console.trace('[MusicLibrary] refresh called')
                console.log('[MusicLibrary] invoke load_music_library start')

                const tracks = await invoke<TrackInfo[]>('load_music_library')

                console.log('[MusicLibrary] invoke load_music_library end', tracks.length)

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