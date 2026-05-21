<script>
    import { playerState } from '$lib/player.svelte'
    import { invoke } from '@tauri-apps/api/core'
    import Progress from './Progress.svelte'
    import Slider from './Slider.svelte'

    let currentTime = $state(0)
    let volume = $state(0)
    let muted = $state(false)
    let silder = $state(null)

    async function switchTrack(step) {
        if (!playerState.playlist.length || !playerState.current) return

        const len = playerState.playlist.length
        const currentIndex = playerState.playlist.indexOf(playerState.current)

        const newIndex = (currentIndex + step + len) % len

        playerState.current = playerState.playlist[newIndex]
        playerState.progress = 0
        playerState.isPlaying = true
    }

    export const next = () => switchTrack(1)
    export const prev = () => switchTrack(-1)

    export async function toggle() {
        playerState.isPlaying = await invoke('toggle_music')
    }

    let volumeIcon = $derived(getVolumeIcon(volume, muted))

    function getVolumeIcon(volume, muted) {
        if (muted) return 'volume_off--rounded'
        if (volume === 0) return 'volume_mute--rounded'
        return volume > 50 ? 'volume_up--rounded' : 'volume_down--rounded'
    }

    async function seek(time) {
        console.log(time)
        stopPolling()
        await invoke('set_current_time', { time })
        currentTime = time
        if (playerState.isPlaying) startPolling()
    }
    
    let pollTimer

    function startPolling() {
        stopPolling()
        pollTimer = setInterval(async () => {
            try {
                currentTime = await invoke('current_time')
            } catch {}
        }, 250)
    }

    function stopPolling() {
        if (pollTimer) {
            clearInterval(pollTimer)
            pollTimer = null
        }
    }

    $effect(() => {
        if (playerState.isPlaying) {
            startPolling()
        } else {
            stopPolling()
        }
        return stopPolling
    })

    $effect(() => {
        if (playerState.current) {
            currentTime = 0
        }
    })
    
    function formatTime(seconds) {
        if (!seconds || !isFinite(seconds)) return '00:00'
        const m = Math.floor(seconds / 60)
        const s = Math.floor(seconds % 60)
        return `${String(m).padStart(2, '0')}:${String(s).padStart(2, '0')}`
    }
</script>

<mdui-bottom-app-bar
    class="relative flex flex-col h-24 px-4 pb-2"
    style="--z-index: 10"
>
        <div class="absolute top-0 left-0 w-full flex items-center gap-2 px-2 -translate-y-1/2">
        <span class="text-xs tabular-nums text-black min-w-10 text-right">
            {formatTime(currentTime)}
        </span>
        <div class="flex-1">
            <Slider
                bind:value={currentTime}
                bind:this={silder}
                oninput={e => seek(e.target.value)}
                duration = {playerState.duration}
            />
        </div>
        <span class="text-xs tabular-nums text-black min-w-10">
            {formatTime(playerState.duration)}
        </span>
    </div>

    <div class="info">
        <div>{playerState.current?.title}</div>
        <div class="artist">{playerState.current?.artist}</div>
    </div>

    <div class="flex items-center justify-between w-full h-full mt-2">
        <div class="flex flex-col justify-center min-w-37.5 overflow-hidden">
            <div class="text-sm font-medium truncate">
                {playerState.current?.title ?? '未在播放'}
            </div>
            <div class="text-xs opacity-70 truncate">
                {playerState.current?.artist ?? '未知艺术家'}
            </div>
        </div>

        <div class="flex items-center gap-2">
            <mdui-button-icon
                icon="skip_previous--rounded"
                role="button"
                tabindex="0"
                onclick={prev}
                onkeydown={e => e.key === 'Enter' && prev()}
            ></mdui-button-icon>
            <mdui-button-icon
                variant="filled"
                icon={playerState.isPlaying
                    ? 'pause--rounded'
                    : 'play_arrow--rounded'}
                role="button"
                tabindex="0"
                onclick={toggle}
                onkeydown={e => e.key === 'Enter' && toggle()}
            >
            </mdui-button-icon>
            <mdui-button-icon
                icon="skip_next--rounded"
                role="button"
                tabindex="0"
                onclick={next}
                onkeydown={e => e.key === 'Enter' && next()}
            ></mdui-button-icon>
        </div>

        <div class="flex items-center min-w-37.5 justify-end">
            <mdui-button-icon
                icon={volumeIcon}
                onclick={() => (muted = !muted)}
                onkeydown={e => e.key === 'Enter' && (muted = !muted)}
                role="button"
                tabindex="0"
                class="text-lg opacity-70"
            >
            </mdui-button-icon>
            <!-- <Slider value="100" class="w-32" /> -->
            <Progress bind:value={volume} class="w-32" />
        </div>
    </div>
</mdui-bottom-app-bar>
