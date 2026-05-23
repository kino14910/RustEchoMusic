<script>
    import { player } from '$lib/player.svelte'
    import Progress from './Progress.svelte'
    import Slider from './Slider.svelte'

    let slider = $state(null)
    let volumeIcon = $derived(
        getVolumeIcon(player.volume, player.muted),
    )

    function getVolumeIcon(volume, muted) {
        if (muted) return 'volume_off--rounded'
        if (volume === 0) return 'volume_mute--rounded'
        return volume > 50 ? 'volume_up--rounded' : 'volume_down--rounded'
    }

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
    <div
        class="absolute top-0 left-0 w-full flex items-center gap-2 px-2 -translate-y-1/2"
    >
        <span class="text-xs tabular-nums text-black min-w-10 text-right">
            {formatTime(player.currentTime)}
        </span>
        <div class="flex-1">
            <Slider
                bind:value={player.currentTime}
                bind:this={slider}
                oninput={e => player.seek(+e.target.value)}
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
                src={player.currentTrack?.cover ??
                    'default_cover.png'}
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
                onkeydown={e => e.key === 'Enter' && player.prev()}
            ></mdui-button-icon>
            <mdui-button-icon
                variant="filled"
                icon={player.playing
                    ? 'pause--rounded'
                    : 'play_arrow--rounded'}
                role="button"
                tabindex="0"
                onclick={() => player.toggle()}
                onkeydown={e => e.key === 'Enter' && player.toggle()}
            >
            </mdui-button-icon>
            <mdui-button-icon
                icon="skip_next--rounded"
                role="button"
                tabindex="0"
                onclick={() => player.next()}
                onkeydown={e => e.key === 'Enter' && player.next()}
            ></mdui-button-icon>
        </div>
        <div class="flex flex-1 items-center min-w-37.5 justify-end">
            <mdui-button-icon
            icon="playlist_play--rounded"
            ></mdui-button-icon>
            <mdui-button-icon
                icon={volumeIcon}
                onclick={() => (player.muted = !player.muted)}
                onkeydown={e =>
                    e.key === 'Enter' &&
                    (player.muted = !player.muted)}
                role="button"
                tabindex="0"
                class="text-lg opacity-70"
            ></mdui-button-icon>
            <Progress bind:value={player.volume} class="w-32" />
        </div>
    </div>
</mdui-bottom-app-bar>
