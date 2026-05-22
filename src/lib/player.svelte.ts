import { invoke } from "@tauri-apps/api/core"


class PlayerState {
    current = $state<TrackInfo | null>(null)
    playing = $state(false)
    playlist = $state<any[]>([])
    currentTime = $state(0)
    volume = $state(80)
    muted = $state(false)

    #pollTimer: any = null

    startPolling = () => {
        this.stopPolling()
        this.#pollTimer = setInterval(async () => {
            if (!this.playing) {
                this.stopPolling()
                return
            }
            this.currentTime = await invoke('current_time')
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

    setVolume = async () => {
        await invoke('set_volume', { volume: this.volume })
    }

    switchTrack = async (step: number) => {
        if (!this.playlist.length || !this.current) return

        const len = this.playlist.length
        const currentIndex = this.playlist.indexOf(this.current)
        const newIndex = (currentIndex + step + len) % len

        this.current = this.playlist[newIndex]
        this.currentTime = 0
        this.playing = true

        this.startPolling()
    }

    next = () => this.switchTrack(1)
    prev = () => this.switchTrack(-1)
}

export interface TrackInfo {
    title: string
    artist: string
    album: string
    duration: number
    sample_rate?: number
    cover?: string
}

export const playerState = new PlayerState()