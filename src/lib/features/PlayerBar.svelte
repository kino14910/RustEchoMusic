<script lang="ts">
    // Mdui 的 M2 Slider,用于播放进度
    import MduiSlider from '$lib/components/base/MduiSlider.svelte'
    // M3 自定义 Slider
    import IconButton from '$lib/components/base/IconButton.svelte'
    import Slider from '$lib/components/base/Slider.svelte'
    import { trackCovers } from '$lib/state/covers.svelte'
    import { player } from '$lib/state/player.svelte'
    import { formatTime } from '$lib/utils'

    let slider = $state<MduiSlider | null>(null)
    let currentCover = $state<string | null>(null)

    let volumeIcon = $derived(getVolumeIcon(player.volume, player.muted))

    function getVolumeIcon(volume: number, muted: boolean) {
        if (muted) return 'volume_off--rounded'
        if (volume === 0) return 'volume_mute--rounded'
        return volume > 50 ? 'volume_up--rounded' : 'volume_down--rounded'
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

{#snippet progressArea()}
    <div
        class="absolute top-2 left-0 w-full flex items-center -translate-y-1/2"
    >
        <span class="text-xs tabular-nums text-black min-w-10 text-right">
            {formatTime(player.currentTime)}
        </span>

        <div class="flex-1">
            <MduiSlider
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
{/snippet}

{#snippet nowPlayingInfo()}
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
{/snippet}

{#snippet transportControls()}
    <div class="flex items-center gap-2">
        <IconButton
            icon="skip_previous--rounded"
            onclick={() => player.prev()}
        />

        <IconButton
            variant="filled"
            icon={player.playing ? 'pause--rounded' : 'play_arrow--rounded'}
            onclick={() => player.toggle()}
        />

        <IconButton icon="skip_next--rounded" onclick={() => player.next()} />
    </div>
{/snippet}

{#snippet volumeControls()}
    <div class="flex flex-1 items-center min-w-37.5 justify-end">
        <IconButton icon="playlist_play--rounded" />

        <IconButton
            icon={volumeIcon}
            onclick={() => (player.muted = !player.muted)}
            class="text-lg opacity-70"
        />

        <Slider bind:value={player.volume} />
    </div>
{/snippet}

<mdui-bottom-app-bar
    class="relative flex flex-col h-24 px-4 pb-2"
    style="--z-index: 10"
>
    {@render progressArea()}
    <div class="flex items-center w-full h-full mt-2">
        {@render nowPlayingInfo()}
        {@render transportControls()}
        {@render volumeControls()}
    </div>
</mdui-bottom-app-bar>
