
import { invoke } from '@tauri-apps/api/core'
import type { Track } from '../types/music'

class TrackCovers {
    covers = $state<Record<string, string | null>>({})
    loading = $state<Record<string, boolean>>({})

    private promises = new Map<string, Promise<string | null>>()

    async load(track: Track | null | undefined): Promise<string | null> {
        if (!track?.path) return null

        if (this.covers[track.path] !== undefined) {
            return this.covers[track.path]
        }

        if (this.promises.has(track.path)) {
            return await this.promises.get(track.path)!
        }

        this.loading[track.path] = true

        const promise = invoke<string | null>('get_track_cover', {
            fullPath: track.path
        })
            .then((cover) => {
                this.covers[track.path] = cover
                return cover
            })
            .catch((err) => {
                console.error('加载封面失败:', track.path, err)
                this.covers[track.path] = null
                return null
            })
            .finally(() => {
                this.loading[track.path] = false
                this.promises.delete(track.path)
            })

        this.promises.set(track.path, promise)

        return await promise
    }

    get(track: Track | null | undefined): string | null {
        if (!track?.path) return null
        return this.covers[track.path] ?? null
    }
}

export const trackCovers = new TrackCovers()