<script lang="ts">
    import { player, type TrackInfo } from '$lib/player.svelte'
    import { trackCovers } from '$lib/trackCovers.svelte'
    import 'mdui/components/list-item.js'
    import 'mdui/components/list.js'

    let { tracks }: { tracks: TrackInfo[] } = $props()

    function formatDuration(seconds: number): string {
        const mins = Math.floor(seconds / 60)
        const secs = Math.floor(seconds % 60)
        return `${mins}:${secs.toString().padStart(2, '0')}`
    }

    function lazyCover(node: HTMLElement, track: TrackInfo) {
        let currentTrack = track

        const observer = new IntersectionObserver(
            entries => {
                const entry = entries[0]

                if (entry.isIntersecting) {
                    void trackCovers.load(currentTrack)
                    observer.disconnect()
                }
            },
            {
                rootMargin: '300px',
            },
        )

        observer.observe(node)

        return {
            update(nextTrack: TrackInfo) {
                currentTrack = nextTrack
            },
            destroy() {
                observer.disconnect()
            },
        }
    }

    function handlePlay(clickedTrack: TrackInfo, clickedIndex: number) {
        player.playlist = tracks
        player.playByIndex(clickedIndex)

        void trackCovers.load(clickedTrack)
    }
</script>

<mdui-list class="flex flex-col w-full">
    <mdui-list-subheader
        class="flex pl-4 pr-6 text-xs uppercase tracking-wider font-semibold themed-text-secondary themed-border-b *:justify-center *:self-center"
        noninteractive
    >
        <div class="w-12 text-center flex">#</div>
        <div class="flex-1 pl-2 text-left">标题</div>
        <div class="flex-1 text-left hidden md:block">专辑</div>
        <div class="w-16">时长</div>
    </mdui-list-subheader>

    <div class="flex flex-col gap-1 mt-2">
        {#each tracks as track, index (track.path)}
            {@const isCurrent =
                player.playlist[player.currentIndex]?.path === track.path}
            {@const cover = trackCovers.get(track)}

            <mdui-list-item
                class="group w-full rounded-xl transition-all duration-200 text-left overflow-hidden outline-none
             active:scale-[0.995] themed-item
             {isCurrent ? 'themed-item-active' : ''}"
                ondblclick={() => handlePlay(track, index)}
                role="button"
                tabindex="0"
            >
                <mdui-ripple></mdui-ripple>

                <div class="flex items-center w-full">
                    <div
                        class="w-12 flex justify-center items-center text-sm themed-text-secondary z-10 shrink-0"
                    >
                        {#if isCurrent && player.playing}
                            <div class="flex items-end gap-0.5 h-3.5">
                                <span
                                    class="w-0.5 themed-bar h-full animate-[bounce_0.8s_infinite_alternate]"
                                ></span>
                                <span
                                    class="w-0.5 themed-bar h-full animate-[bounce_0.8s_infinite_alternate_0.2s]"
                                ></span>
                                <span
                                    class="w-0.5 themed-bar h-full animate-[bounce_0.8s_infinite_alternate_0.4s]"
                                ></span>
                            </div>
                        {:else}
                            <span
                                class={isCurrent
                                    ? 'font-bold themed-text-accent'
                                    : 'group-hover:text-[rgb(var(--mdui-color-on-surface))]'}
                            >
                                {index + 1}
                            </span>
                        {/if}
                    </div>

                    <div
                        class="flex-1 flex items-center gap-4 pl-3 overflow-hidden z-10"
                    >
                        <div
                            use:lazyCover={track}
                            class="w-10 h-10 rounded-md overflow-hidden themed-surface-container shrink-0 transition-transform duration-300 group-hover:scale-105 group-hover:shadow-md"
                        >
                            {#if cover}
                                <img
                                    src={cover}
                                    alt="cover"
                                    class="w-full h-full object-cover"
                                    loading="lazy"
                                />
                            {:else}
                                <div
                                    class="w-full h-full flex items-center justify-center text-base themed-surface-high"
                                >
                                    🎵
                                </div>
                            {/if}
                        </div>

                        <div class="flex flex-col min-w-0">
                            <span
                                class="text-sm font-medium truncate {isCurrent
                                    ? 'themed-text-accent'
                                    : 'themed-text-primary'}"
                            >
                                {track.title}
                            </span>
                            <span
                                class="text-xs truncate themed-text-secondary group-hover:text-[rgb(var(--mdui-color-on-surface))] mt-0.5"
                            >
                                {track.artist}
                            </span>
                        </div>
                    </div>

                    <div
                        class="flex-1 text-sm truncate hidden md:block z-10 themed-text-secondary"
                    >
                        {track.album}
                    </div>

                    <div
                        class="w-16 text-sm font-mono z-10 themed-text-secondary group-hover:text-[rgb(var(--mdui-color-on-surface))] shrink-0"
                    >
                        {formatDuration(track.duration)}
                    </div>
                </div>
            </mdui-list-item>
        {/each}
    </div>
</mdui-list>

<style>
    @keyframes bounce {
        from {
            height: 4px;
        }
        to {
            height: 14px;
        }
    }

    .themed-text-primary {
        color: rgb(var(--mdui-color-on-surface));
    }

    .themed-text-secondary {
        color: rgb(var(--mdui-color-on-surface-variant));
    }

    .themed-text-accent {
        color: rgb(var(--mdui-color-primary));
    }

    .themed-border {
        border-color: rgb(var(--mdui-color-outline-variant));
    }

    .themed-border-b {
        border-bottom: 1px solid rgb(var(--mdui-color-outline-variant));
    }

    .themed-surface-container {
        background-color: rgb(var(--mdui-color-surface-container-highest));
    }

    .themed-surface-high {
        background-color: rgb(var(--mdui-color-surface-container-high));
    }

    .themed-bar {
        background-color: rgb(var(--mdui-color-primary));
    }

    .themed-item {
        background-color: transparent;
        border: 1px solid transparent;
    }

    .themed-item:hover {
        background-color: rgb(var(--mdui-color-surface-container));
        box-shadow:
            0 4px 6px -1px rgba(0, 0, 0, 0.1),
            0 2px 4px -1px rgba(0, 0, 0, 0.06);
    }

    .themed-item-active {
        background-color: rgb(var(--mdui-color-surface-container));
        border-color: rgb(var(--mdui-color-outline-variant));
    }
</style>
