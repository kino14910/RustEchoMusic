import { invoke } from "@tauri-apps/api/core"
import type { Track } from "../types/types"
import { recentlyPlayed } from "./recent.svelte"


class Player {
    currentIndex = $state<number>(-1)
    playing = $state(false)
    playlist = $state<any[]>([])
    currentTime = $state(0)
    muted = $state(false)

    currentTrack = $derived<Track | null>(
        this.currentIndex >= 0 && this.currentIndex < this.playlist.length
            ? this.playlist[this.currentIndex]
            : null
    )
    // duration = $derived(this.currentTrack?.duration ?? 0)

    #volume = $state(80);
    get volume() { return this.#volume }
    set volume(volume: number) {
        this.#volume = volume
        invoke('set_volume', { volume }).catch(console.error)
    }

    #pollTimer: any = null

    startPolling = () => {
        this.stopPolling()
        this.#pollTimer = setInterval(async () => {
            if (!this.playing) {
                this.stopPolling()
                return
            }
            this.currentTime = await invoke('current_time')
            if (this.currentTrack && this.currentTime >= this.currentTrack.duration) {
                this.next()
            }
        }, 250)
    }

    stopPolling = () => {
        if (this.#pollTimer) {
            clearInterval(this.#pollTimer)
            this.#pollTimer = null
        }
    }

    toggle = async () => {
        this.playing = await invoke('toggle_music')
        if (this.playing) {
            this.startPolling()
        } else {
            this.stopPolling()
        }
    }

    seek = async (time: number) => {
        this.stopPolling()
        await invoke('set_current_time', { time })
        this.currentTime = time
        if (this.playing) this.startPolling()
    }

    playByIndex = async (index: number) => {
        if (index < 0 || index >= this.playlist.length) return

        try {
            const track = this.playlist[index]

            if (!track) {
                return
            }
            
            this.currentIndex = index
            this.currentTime = 0
            this.playing = true
            await invoke('play_music', { fullPath: track.path })

            recentlyPlayed.add(track)

            this.startPolling()
        } catch (err) {
            console.error('切换歌曲失败:', err)
        }
    }

    switchTrack = async (step: number) => {
        const len = this.playlist.length
        if (len === 0) return
        
        this.stopPolling()
        const newIndex = (this.currentIndex + step + len) % len
        this.playByIndex(newIndex)
    }

    next = () => this.switchTrack(1)
    prev = () => this.switchTrack(-1)
}

export const player = new Player()