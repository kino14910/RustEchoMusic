<script lang="ts">
    import { player } from '$lib/player.svelte'
    import { trackCovers } from '$lib/trackCovers.svelte'
    import Progress from './Progress.svelte'
    import Slider from './Slider.svelte'

    let slider = $state<Slider | null>(null)
    let currentCover = $state<string | null>(null)

    let volumeIcon = $derived(getVolumeIcon(player.volume, player.muted))

    function getVolumeIcon(volume: number, muted: boolean) {
        if (muted) return 'volume_off--rounded'
        if (volume === 0) return 'volume_mute--rounded'
        return volume > 50 ? 'volume_up--rounded' : 'volume_down--rounded'
    }

    function formatTime(seconds: number) {
        if (!seconds || !isFinite(seconds)) return '00:00'
        const m = Math.floor(seconds / 60)
        const s = Math.floor(seconds % 60)
        return `${String(m).padStart(2, '0')}:${String(s).padStart(2, '0')}`
    }

    function handleSeekInput(e: Event): void {
        const target = e.currentTarget as HTMLElement & {
            value: number | string
        }

        player.seek(Number(target.value))
    }

    $effect(() => {
        const track = player.currentTrack

        if (!track) {
            currentCover = null
            return
        }

        currentCover = trackCovers.get(track)

        trackCovers.load(track).then(cover => {
            if (player.currentTrack?.path === track.path) {
                currentCover = cover
            }
        })
    })
</script>

<mdui-bottom-app-bar
    class="relative flex flex-col h-24 px-4 pb-2"
    style="--z-index: 10"
>
    <div
        class="absolute top-2 left-0 w-full flex items-center -translate-y-1/2"
    >
        <span class="text-xs tabular-nums text-black min-w-10 text-right">
            {formatTime(player.currentTime)}
        </span>
        <div class="flex-1">
            <Slider
                bind:value={player.currentTime}
                bind:this={slider}
                oninput={handleSeekInput}
                duration={player.currentTrack?.duration ?? 0}
            />
        </div>
        <span class="text-xs tabular-nums text-black min-w-10">
            {formatTime(player.currentTrack?.duration ?? 0)}
        </span>
    </div>

    <div class="flex items-center w-full h-full mt-2">
        <div class="flex flex-1 min-w-37.5 gap-2">
            <img
                src={currentCover ?? '/default_cover.png'}
                alt="Album Cover"
                class="w-16 h-16 rounded object-cover shadow-sm bg-gray-200"
            />
            <div class="flex flex-col justify-center overflow-hidden">
                <div class="text-sm font-medium truncate">
                    {player.currentTrack?.title ?? '未在播放'}
                </div>
                <div class="text-xs opacity-70 truncate">
                    {player.currentTrack?.artist ?? '未知艺术家'}
                </div>
            </div>
        </div>

        <div class="flex items-center gap-2">
            <mdui-button-icon
                icon="skip_previous--rounded"
                role="button"
                tabindex="0"
                onclick={() => player.prev()}
                onkeydown={(e: KeyboardEvent) =>
                    e.key === 'Enter' && player.prev()}
            ></mdui-button-icon>
            <mdui-button-icon
                variant="filled"
                icon={player.playing ? 'pause--rounded' : 'play_arrow--rounded'}
                role="button"
                tabindex="0"
                onclick={() => player.toggle()}
                onkeydown={(e: KeyboardEvent) =>
                    e.key === 'Enter' && player.toggle()}
            >
            </mdui-button-icon>
            <mdui-button-icon
                icon="skip_next--rounded"
                role="button"
                tabindex="0"
                onclick={() => player.next()}
                onkeydown={(e: KeyboardEvent) =>
                    e.key === 'Enter' && player.next()}
            ></mdui-button-icon>
        </div>
        <div class="flex flex-1 items-center min-w-37.5 justify-end">
            <mdui-button-icon icon="playlist_play--rounded"></mdui-button-icon>
            <mdui-button-icon
                icon={volumeIcon}
                onclick={() => (player.muted = !player.muted)}
                onkeydown={(e: KeyboardEvent) =>
                    e.key === 'Enter' && (player.muted = !player.muted)}
                role="button"
                tabindex="0"
                class="text-lg opacity-70"
            ></mdui-button-icon>
            <Progress bind:value={player.volume} />
        </div>
    </div>
</mdui-bottom-app-bar>
