import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import type { Track } from '../types/music'
import { settings } from './settings.svelte'

class MusicLibrary {
    tracks = $state<Track[]>([])
    isLoading = $state(false)
    error = $state<string | null>(null)

    initialized = false

    private refreshPromise: Promise<Track[]> | null = null

    constructor() {
        void this.setupListener()
    }

    private async setupListener() {
        await listen<Track[]>(
            'library:refreshed',
            event => {
                this.tracks = event.payload
            },
        )
    }

    async load(options: { force?: boolean } = {}): Promise<Track[]> {
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

    async scan(): Promise<Track[]> {
        try {
            this.isLoading = true
            this.error = null

            const tracks = await invoke<Track[]>(
                'scan_music_directories',
                {
                    dirs: settings.data.libraryDirs,
                }
            )

            this.tracks = tracks

            return tracks
        } catch (err) {
            console.error('扫描媒体库失败:', err)

            this.error = String(err)

            return this.tracks
        } finally {
            this.isLoading = false
        }
    }
}

export const musicLibrary = new MusicLibrary()