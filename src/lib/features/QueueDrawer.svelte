<script lang="ts">
    import IconButton from '$lib/components/base/IconButton.svelte'
    import PlayingIndicator from '$lib/components/base/PlayingIndicator.svelte'
    import { trackCovers } from '$lib/state/covers.svelte'
    import { player } from '$lib/state/player.svelte'
    import { formatDuration } from '$lib/utils'
    import '@mdui/icons/equalizer--rounded.js'
    import '@mdui/icons/play-arrow--rounded.js'

    function handlePlay(index: number) {
        player.playTrackInQueue(index)
    }

    function handleRemove(id: string, e: Event) {
        e.stopPropagation()
        player.removeTrack(id)
    }

    function lazyCover(node: HTMLImageElement, track: any) {
        let currentTrack = track
        const observer = new IntersectionObserver(
            entries => {
                const entry = entries[0]
                if (entry.isIntersecting) {
                    trackCovers.load(currentTrack).then(cover => {
                        if (cover) {
                            node.src = cover
                        }
                    })
                    observer.disconnect()
                }
            },
            { rootMargin: '100px' }
        )
        observer.observe(node)
        return {
            update(nextTrack: any) {
                currentTrack = nextTrack
            },
            destroy() {
                observer.disconnect()
            }
        }
    }
</script>

<mdui-navigation-drawer
    open={player.queueOpen}
    placement="right"
    close-on-overlay-click
    close-on-esc
    onclose={() => { player.queueOpen = false }}
    class="w-96 max-w-full flex flex-col h-full bg-[rgb(var(--mdui-color-surface-container))] border-l border-[rgb(var(--mdui-color-outline-variant))]"
>
    <div class="flex items-center justify-between p-4 border-b border-[rgb(var(--mdui-color-outline-variant))]">
        <div class="flex items-center gap-2">
            <span class="text-lg font-semibold text-[rgb(var(--mdui-color-on-surface))]">播放队列</span>
            <span class="px-2 py-0.5 text-xs rounded-full bg-[rgb(var(--mdui-color-secondary-container))] text-[rgb(var(--mdui-color-on-secondary-container))] font-medium">
                {player.playlist.length}
            </span>
        </div>
        <IconButton icon="close--rounded" onclick={() => { player.queueOpen = false }} />
    </div>

    <div class="flex-1 overflow-y-auto p-2 space-y-1">
        {#if player.playlist.length === 0}
            <div class="flex flex-col items-center justify-center h-64 text-center p-6 text-[rgb(var(--mdui-color-on-surface-variant))]">
                <span class="text-4xl mb-2">🎵</span>
                <p class="text-sm">队列为空</p>
            </div>
        {:else}
            {#each player.playlist as track, index (track.id)}
                {@const isCurrent = player.currentIndex === index}
                <div
                    role="button"
                    tabindex="0"
                    ondblclick={() => handlePlay(index)}
                    onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') handlePlay(index) }}
                    class="group flex items-center gap-3 p-2.5 rounded-lg cursor-pointer transition-all duration-200 select-none outline-none
                           {isCurrent 
                               ? 'bg-[rgb(var(--mdui-color-secondary-container))] text-[rgb(var(--mdui-color-on-secondary-container))] border border-[rgb(var(--mdui-color-outline-variant))]' 
                               : 'hover:bg-[rgb(var(--mdui-color-surface-container-high))] text-[rgb(var(--mdui-color-on-surface))] border border-transparent'}"
                >
                    <div class="relative w-10 h-10 rounded overflow-hidden bg-[rgb(var(--mdui-color-surface-container-highest))] shrink-0 flex items-center justify-center">
                        <img
                            use:lazyCover={track}
                            src={trackCovers.get(track) || '/default_cover.png'}
                            alt="cover"
                            class="w-full h-full object-cover"
                        />
                        {#if isCurrent}
                            <div class="absolute inset-0 bg-black/40 flex items-center justify-center text-[rgb(var(--mdui-color-inverse-primary-light))] text-lg">
                                {#if player.playing}
                                    <PlayingIndicator color="rgb(var(--mdui-color-inverse-primary-light))"/>
                                {:else}
                                    <mdui-icon-play-arrow--rounded></mdui-icon-play-arrow--rounded>
                                {/if}
                            </div>
                        {/if}
                    </div>

                    <div class="flex-1 min-w-0">
                        <div class="text-sm font-medium truncate {isCurrent ? 'text-[rgb(var(--mdui-color-primary))] font-semibold' : ''}">
                            {track.title}
                        </div>
                        <div class="text-xs truncate text-[rgb(var(--mdui-color-on-surface-variant))] mt-0.5">
                            {track.artist}
                        </div>
                    </div>

                    <div class="flex items-center gap-1.5 shrink-0">
                        <span class="text-xs font-mono text-[rgb(var(--mdui-color-on-surface-variant))]">
                            {formatDuration(track.duration)}
                        </span>
                        <IconButton
                            icon="close--rounded"
                            class="opacity-0 group-hover:opacity-100 transition-opacity duration-200 text-xs text-[rgb(var(--mdui-color-on-surface-variant))] hover:text-red-500"
                            onclick={(e: MouseEvent) => handleRemove(track.id, e)}
                        />
                    </div>
                </div>
            {/each}
        {/if}
    </div>
</mdui-navigation-drawer>
