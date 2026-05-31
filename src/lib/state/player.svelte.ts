import { invoke } from "@tauri-apps/api/core"
import { load } from "@tauri-apps/plugin-store"
import type { Track } from "../types/music"
import { recentlyPlayed } from "./recent.svelte"

type PlayMode = 'list' | 'single' | 'shuffle'

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
    #loadToken = 0
    #storePromise: ReturnType<typeof load> | null = null

    constructor() {
        this.#setupMediaSession()
    }

    #setupMediaSession() {
        if (!('mediaSession' in navigator)) return

        navigator.mediaSession.setActionHandler('play', () => this.resume())
        navigator.mediaSession.setActionHandler('pause', () => this.pause())
        navigator.mediaSession.setActionHandler('previoustrack', () => this.prev())
        navigator.mediaSession.setActionHandler('nexttrack', () => this.next())
        navigator.mediaSession.setActionHandler('seekto', details => {
            const seekTime = details.seekTime
            if (seekTime == null) return
            void this.seek(seekTime)
        })
        navigator.mediaSession.setActionHandler('seekforward', () => {
            if (!this.currentTrack) return
            void this.seek(Math.min(this.currentTime + 10, this.currentTrack.duration))
        })
        navigator.mediaSession.setActionHandler('seekbackward', () => {
            if (!this.currentTrack) return
            void this.seek(Math.max(this.currentTime - 10, 0))
        })
    }

    #updateMediaMetadata() {
        if (!('mediaSession' in navigator) || !this.currentTrack) return

        navigator.mediaSession.metadata = new MediaMetadata({
            title: this.currentTrack.title,
            artist: this.currentTrack.artist ?? '未知歌手',
            album: this.currentTrack.album ?? '未知专辑',
            artwork: this.#buildArtwork(),
        })
    }

    #buildArtwork() {
        const cover = this.currentTrack?.cover
        return cover ? [{ src: cover }] : []
    }

    #updatePlaybackState() {
        if (!('mediaSession' in navigator)) return
        navigator.mediaSession.playbackState = this.playing ? 'playing' : 'paused'
    }

    #updatePositionState() {
        const track = this.currentTrack
        if (!track || !('mediaSession' in navigator) || !('setPositionState' in navigator.mediaSession)) return

        navigator.mediaSession.setPositionState({
            duration: track.duration,
            playbackRate: this.playing ? 1 : 0,
            position: this.currentTime,
        })
    }

    #syncMediaSession() {
        this.#updateMediaMetadata()
        this.#updatePlaybackState()
        this.#updatePositionState()
    }

    #clearMediaSession() {
        if (!('mediaSession' in navigator)) return
        navigator.mediaSession.metadata = null
        navigator.mediaSession.playbackState = 'none'
    }

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
        void this.#persist()
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

    async #getStore() {
        if (!this.#storePromise) {
            this.#storePromise = load(STORE_NAME)
        }
        return this.#storePromise
    }

    replacePlaylistAndPlay(tracks: Track[], targetId: string) {
        if (tracks.length === 0) {
            this.playlist = []
            this.currentIndex = -1
            this.playing = false
            this.playbackHistory = []
            this.#clearMediaSession()
            this.#stopPolling()
            invoke('toggle_music').catch(console.error)
            return
        }

        const nextQueue = [...tracks]
        const index = nextQueue.findIndex(t => t.id === targetId)
        const nextIndex = index !== -1 ? index : 0

        this.playbackHistory = []
        this.playlist = nextQueue
        this.currentIndex = nextIndex
        void this.#loadAndPlay()
        void this.#persist()
    }

    appendTrack(track: Track) {
        if (this.playlist.some(t => t.id === track.id)) return
        this.playlist = [...this.playlist, track]
    }

    insertNext(track: Track) {
        const existingIndex = this.playlist.findIndex(item => item.id === track.id)
        if (existingIndex === this.currentIndex && this.currentIndex !== -1) return

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
            this.#clearMediaSession()
            this.#stopPolling()
            invoke('toggle_music').catch(console.error)
            return
        }

        if (isCurrent) {
            const nextIndex = Math.min(removeIndex, nextQueue.length - 1)
            this.playlist = nextQueue
            this.currentIndex = nextIndex
            void this.#loadAndPlay()
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
        void this.#loadAndPlay()
        void this.#persist()
    }

    cyclePlayMode() {
        const modes: PlayMode[] = ['list', 'single', 'shuffle']
        const currentModeIndex = modes.indexOf(this.playMode)
        this.playMode = modes[(currentModeIndex + 1) % modes.length]
        void this.#persist()
    }

    toggleQueue() {
        this.queueOpen = !this.queueOpen
    }

    #getNextIndex(): number {
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

    #getPrevIndex(): number {
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
        this.currentIndex = this.#getNextIndex()
        void this.#loadAndPlay()
    }

    prev() {
        if (this.playlist.length === 0) return
        if (this.playbackHistory.length > 0) {
            const lastId = this.playbackHistory.pop()
            const index = this.playlist.findIndex(t => t.id === lastId)
            if (index !== -1) {
                this.currentIndex = index
                void this.#loadAndPlay()
                return
            }
        }
        this.currentIndex = this.#getPrevIndex()
        void this.#loadAndPlay()
    }

    resume = async () => {
        try {
            await invoke('resume_music')
            this.playing = true
            this.#syncMediaSession()
            this.#startPolling()
        } catch (err) {
            console.error(err)
        }
    }

    pause = async () => {
        try {
            await invoke('pause_music')
            this.playing = false
            this.#syncMediaSession()
            this.#stopPolling()
        } catch (err) {
            console.error(err)
        }
    }

    toggle = async () => {
        if (this.playing) {
            await this.pause()
        } else {
            await this.resume()
        }
    }

    seek = async (time: number) => {
        this.#stopPolling()
        await invoke('set_current_time', { time })
        this.currentTime = time
        this.#updatePositionState()
        if (this.playing) {
            this.#startPolling()
        }
    }

    async loadState() {
        try {
            const store = await this.#getStore()
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

    async #persist() {
        try {
            const store = await this.#getStore()
            await store.set("playlist", this.playlist.map(t => ({ ...t, cover: null })))
            await store.set("currentIndex", this.currentIndex)
            await store.set("playMode", this.playMode)
            await store.set("volume", this.#volume)
            await store.save()
        } catch (err) {
            console.error(err)
        }
    }

    #onTrackStarted(track: Track) {
        this.playing = true
        this.#syncMediaSession()
        recentlyPlayed.add(track)
    }

    async #loadAndPlay() {
        const track = this.currentTrack
        if (!track) return

        const currentToken = ++this.#loadToken
        this.#stopPolling()

        try {
            this.currentTime = 0
            if (track.path) {
                await invoke('play_music', { fullPath: track.path })
            } else {
                await invoke('play_online_music', { url: track.url, id: track.id })
            }

            if (currentToken !== this.#loadToken) return

            this.#onTrackStarted(track)
            this.#startPolling()
        } catch (err) {
            if (currentToken === this.#loadToken) {
                this.playing = false
                console.error(err)
            }
        }
    }

    #startPolling = () => {
        this.#stopPolling()
        this.#pollTimer = setInterval(async () => {
            if (!this.playing) {
                this.#stopPolling()
                return
            }
            try {
                this.currentTime = await invoke<number>('current_time')
                if (this.currentTrack && this.currentTime >= this.currentTrack.duration - 0.5) {
                    if (this.playMode === 'single') {
                        this.currentTime = 0
                        void this.#loadAndPlay()
                    } else {
                        this.next()
                    }
                }
            } catch (e) {
                this.#stopPolling()
            }
        }, 250)
    }

    #stopPolling = () => {
        if (this.#pollTimer) {
            clearInterval(this.#pollTimer)
            this.#pollTimer = null
        }
    }

    destroy() {
        this.#stopPolling()
    }
}

export const player = new Player()