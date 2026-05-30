import { invoke } from "@tauri-apps/api/core"
import { load } from "@tauri-apps/plugin-store"
import type { Track } from "../types/music"
import { recentlyPlayed } from "./recent.svelte"

type PlayMode = 'list' | 'single' | 'shuffle'

interface PersistedState {
    playlist: Track[]
    currentIndex: number
    playMode: PlayMode
    volume: number
}

const STORE_NAME = "player-state.json"

class Player {
    playlist = $state<Track[]>([])
    currentIndex = $state<number>(-1)
    playing = $state<boolean>(false)
    currentTime = $state<number>(0)
    playMode = $state<PlayMode>('list')
    queueOpen = $state<boolean>(false)
    playbackHistory: string[] = []
    #muted = $state<boolean>(false)
    #volume = $state<number>(80)
    #previousVolume = 80
    #pollTimer: any = null
    private loadToken = 0

    constructor() {}

    get currentTrack(): Track | null {
        if (this.currentIndex >= 0 && this.currentIndex < this.playlist.length) {
            return this.playlist[this.currentIndex]
        }
        return null
    }

    get volume() { return this.#volume }
    set volume(volume: number) {
        this.#volume = volume
        if (volume > 0) {
            this.#muted = false
        }
        invoke('set_volume', { volume }).catch(console.error)
        void this.persist()
    }

    get muted() { return this.#muted }
    set muted(value: boolean) {
        this.#muted = value
        if (value) {
            this.#previousVolume = this.#volume
            this.#volume = 0
            invoke('set_volume', { volume: 0 }).catch(console.error)
        } else {
            this.#volume = this.#previousVolume
            invoke('set_volume', { volume: this.#previousVolume }).catch(console.error)
        }
    }

    replacePlaylistAndPlay(tracks: Track[], targetId: string) {
        if (tracks.length === 0) {
            this.playlist = []
            this.currentIndex = -1
            this.playing = false
            this.playbackHistory = []
            this.stopPolling()
            invoke('toggle_music').catch(console.error)
            return
        }

        const nextQueue = [...tracks]
        const index = nextQueue.findIndex(t => t.id === targetId)
        const nextIndex = index !== -1 ? index : 0

        this.playbackHistory = []
        this.playlist = nextQueue
        this.currentIndex = nextIndex
        void this.loadAndPlay()
        void this.persist()
    }

    appendTrack(track: Track) {
        if (this.playlist.some(t => t.id === track.id)) return
        this.playlist = [...this.playlist, track]
    }

    insertNext(track: Track) {
        const existingIndex = this.playlist.findIndex(item => item.id === track.id)
        if (existingIndex === this.currentIndex && this.currentIndex !== -1) {
            return
        }

        let nextQueue = [...this.playlist]
        let nextIndex = this.currentIndex

        if (existingIndex !== -1) {
            nextQueue.splice(existingIndex, 1)
            if (existingIndex < nextIndex) {
                nextIndex--
            }
        }

        if (nextQueue.length === 0 || nextIndex === -1) {
            this.playlist = [track]
            this.currentIndex = 0
            return
        }

        nextQueue.splice(nextIndex + 1, 0, track)
        this.playlist = nextQueue
        this.currentIndex = nextIndex
    }

    removeTrack(id: string) {
        const removeIndex = this.playlist.findIndex(t => t.id === id)
        if (removeIndex === -1) return

        const isCurrent = removeIndex === this.currentIndex
        const nextQueue = this.playlist.filter(t => t.id !== id)

        if (nextQueue.length === 0) {
            this.playlist = []
            this.currentIndex = -1
            this.playing = false
            this.playbackHistory = []
            this.stopPolling()
            invoke('toggle_music').catch(console.error)
            return
        }

        if (isCurrent) {
            const nextIndex = Math.min(removeIndex, nextQueue.length - 1)
            this.playlist = nextQueue
            this.currentIndex = nextIndex
            void this.loadAndPlay()
            return
        }

        let nextIndex = this.currentIndex
        if (removeIndex < nextIndex) {
            nextIndex--
        }
        this.playlist = nextQueue
        this.currentIndex = nextIndex
    }

    playTrackInQueue(index: number) {
        if (index < 0 || index >= this.playlist.length) return
        this.currentIndex = index
        void this.loadAndPlay()
        void this.persist()
    }

    cyclePlayMode() {
        const modes: PlayMode[] = ['list', 'single', 'shuffle']
        const currentModeIndex = modes.indexOf(this.playMode)
        this.playMode = modes[(currentModeIndex + 1) % modes.length]
        void this.persist()
    }

    toggleQueue() {
        this.queueOpen = !this.queueOpen
    }

    private getNextIndex(): number {
        if (this.playlist.length <= 1) return 0
        if (this.playMode === 'shuffle') {
            let nextIndex = this.currentIndex
            while (nextIndex === this.currentIndex) {
                nextIndex = Math.floor(Math.random() * this.playlist.length)
            }
            return nextIndex
        }
        return this.currentIndex >= this.playlist.length - 1 ? 0 : this.currentIndex + 1
    }

    private getPrevIndex(): number {
        if (this.playlist.length <= 1) return 0
        if (this.playMode === 'shuffle') {
            let prevIndex = this.currentIndex
            while (prevIndex === this.currentIndex) {
                prevIndex = Math.floor(Math.random() * this.playlist.length)
            }
            return prevIndex
        }
        return this.currentIndex <= 0 ? this.playlist.length - 1 : this.currentIndex - 1
    }

    next() {
        if (this.playlist.length === 0) return
        const current = this.currentTrack
        if (current) {
            this.playbackHistory.push(current.id)
        }
        this.currentIndex = this.getNextIndex()
        void this.loadAndPlay()
    }

    prev() {
        if (this.playlist.length === 0) return
        if (this.playbackHistory.length > 0) {
            const lastId = this.playbackHistory.pop()
            const index = this.playlist.findIndex(t => t.id === lastId)
            if (index !== -1) {
                this.currentIndex = index
                void this.loadAndPlay()
                return
            }
        }
        this.currentIndex = this.getPrevIndex()
        void this.loadAndPlay()
    }

    toggle = async () => {
        try {
            const isPlayingNow = await invoke<boolean>('toggle_music')
            this.playing = isPlayingNow
            if (this.playing) {
                this.startPolling()
            } else {
                this.stopPolling()
            }
        } catch (err) {
            console.error(err)
        }
    }

    seek = async (time: number) => {
        this.stopPolling()
        await invoke('set_current_time', { time })
        this.currentTime = time
        if (this.playing) this.startPolling()
    }

    async loadState() {
        try {
            const store = await load(STORE_NAME)
            const playlist = await store.get<Track[]>("playlist")
            const currentIndex = await store.get<number>("currentIndex")
            const playMode = await store.get<PlayMode>("playMode")
            const volume = await store.get<number>("volume")

            if (playlist && playlist.length > 0) {
                this.playlist = playlist
            }
            if (typeof currentIndex === "number" && currentIndex >= 0) {
                this.currentIndex = currentIndex
            }
            if (playMode) {
                this.playMode = playMode
            }
            if (typeof volume === "number") {
                this.#volume = volume
                invoke('set_volume', { volume }).catch(console.error)
            }
        } catch (err) {
            console.error(err)
        }
    }

    private async persist() {
        try {
            const store = await load(STORE_NAME)
            await store.set("playlist", this.playlist.map(t => ({ ...t, cover: null })))
            await store.set("currentIndex", this.currentIndex)
            await store.set("playMode", this.playMode)
            await store.set("volume", this.#volume)
            await store.save()
        } catch (err) {
            console.error(err)
        }
    }

    private async loadAndPlay() {
        const track = this.currentTrack
        if (!track) return

        const currentToken = ++this.loadToken
        this.stopPolling()

        try {
            this.currentTime = 0
            if (track.path) {
                await invoke('play_music', { fullPath: track.path })
            } else {
                await invoke('play_online_music', { url: track.url, id: track.id })
            }

            if (currentToken !== this.loadToken) {
                return
            }

            this.playing = true
            recentlyPlayed.add(track)
            this.startPolling()
        } catch (err) {
            if (currentToken === this.loadToken) {
                this.playing = false
                console.error(err)
            }
        }
    }

    private startPolling = () => {
        this.stopPolling()
        this.#pollTimer = setInterval(async () => {
            if (!this.playing) {
                this.stopPolling()
                return
            }
            try {
                this.currentTime = await invoke<number>('current_time')
                if (this.currentTrack && this.currentTime >= this.currentTrack.duration - 0.5) {
                    if (this.playMode === 'single') {
                        this.currentTime = 0
                        void this.loadAndPlay()
                    } else {
                        this.next()
                    }
                }
            } catch (e) {
                this.stopPolling()
            }
        }, 250)
    }

    private stopPolling = () => {
        if (this.#pollTimer) {
            clearInterval(this.#pollTimer)
            this.#pollTimer = null
        }
    }

    destroy() {
        this.stopPolling()
    }
}

export const player = new Player()