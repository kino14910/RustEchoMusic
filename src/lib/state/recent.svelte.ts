
import { invoke } from '@tauri-apps/api/core'
import type { Track } from '../types/music'

class RecentlyPlayed {
    tracks = $state<Track[]>([])
    isLoading = $state(false)
    error = $state<string | null>(null)

    private loadPromise: Promise<Track[]> | null = null

    async load(): Promise<Track[]> {
        if (this.loadPromise) {
            return this.loadPromise
        }

        this.isLoading = true
        this.error = null

        this.loadPromise = invoke<Track[]>('load_recently_played')
            .then((tracks) => {
                this.tracks = tracks
                return tracks
            })
            .catch((err) => {
                console.error('加载最近播放失败:', err)
                this.error = String(err)
                return this.tracks
            })
            .finally(() => {
                this.isLoading = false
                this.loadPromise = null
            })

        return this.loadPromise
    }

    async add(track: Track): Promise<Track[]> {
        const safeTrack: Track = {
            ...track,
            cover: null
        }

        // 先立即更新前端状态，让 UI 马上变化
        this.tracks = [
            safeTrack,
            ...this.tracks.filter((item) => item.path !== safeTrack.path)
        ].slice(0, 100)

        try {
            const tracks = await invoke<Track[]>('add_recently_played', {
                track: safeTrack
            })

            this.tracks = tracks
            return tracks
        } catch (err) {
            console.error('写入最近播放失败:', err)
            this.error = String(err)
            return this.tracks
        }
    }

    async clear() {
        try {
            await invoke('clear_recently_played')
            this.tracks = []
        } catch (err) {
            console.error('清空最近播放失败:', err)
            this.error = String(err)
        }
    }
}

export const recentlyPlayed = new RecentlyPlayed()