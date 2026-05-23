import type { TrackInfo } from '$lib/player.svelte'
import { invoke } from '@tauri-apps/api/core'

class RecentlyPlayed {
    tracks = $state<TrackInfo[]>([])
    isLoading = $state(false)
    error = $state<string | null>(null)

    private loadPromise: Promise<TrackInfo[]> | null = null

    async load(): Promise<TrackInfo[]> {
        if (this.loadPromise) {
            return this.loadPromise
        }

        this.isLoading = true
        this.error = null

        this.loadPromise = invoke<TrackInfo[]>('load_recently_played')
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

    async add(track: TrackInfo): Promise<TrackInfo[]> {
        const safeTrack: TrackInfo = {
            ...track,
            cover: null
        }

        // 先立即更新前端状态，让 UI 马上变化
        this.tracks = [
            safeTrack,
            ...this.tracks.filter((item) => item.path !== safeTrack.path)
        ].slice(0, 100)

        try {
            const tracks = await invoke<TrackInfo[]>('add_recently_played', {
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