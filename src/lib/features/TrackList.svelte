<script lang="ts">
    import PlayingIndicator from '$lib/components/base/PlayingIndicator.svelte'
    import { trackCovers } from '$lib/state/covers.svelte'
    import { player } from '$lib/state/player.svelte'
    import type { Track } from '$lib/types'
    import { formatDuration } from '$lib/utils'
    import 'mdui/components/list-item.js'
    import 'mdui/components/list.js'

    type TrackColumn = 'index' | 'title' | 'album' | 'tags' | 'duration'

    interface Props {
        tracks: Track[]
        columns?: TrackColumn[]
    }

    const columnWidths: Record<TrackColumn, string> = {
        index: '24px',
        title: '1fr',
        album: '1fr',
        tags: 'auto',
        duration: '60px',
    }

    let { tracks, columns = ['index', 'title', 'album', 'duration'] }: Props =
        $props()

    let gridTemplate = $derived(
        columns.map(column => columnWidths[column]).join(' '),
    )

    function lazyCover(track: Track) {
        return (node: HTMLElement) => {
            const observer = new IntersectionObserver(
                entries => {
                    const entry = entries[0]

                    if (entry.isIntersecting) {
                        void trackCovers.load(track)
                        observer.disconnect()
                    }
                },
                {
                    rootMargin: '300px',
                },
            )

            observer.observe(node)

            return () => {
                observer.disconnect()
            }
        }
    }

    function getTrackTags(track: Track): string[] {
        const tags = (track as Track & { tags?: string[] }).tags

        return Array.isArray(tags) ? tags : []
    }

    function handlePlay(clickedTrack: Track) {
        player.replacePlaylistAndPlay(tracks, clickedTrack.id)

        void trackCovers.load(clickedTrack)
    }
</script>

{#snippet trackIndex(index: number, isCurrent: boolean)}
    <div
        class="flex justify-center items-center text-sm themed-text-secondary z-10"
    >
        {#if isCurrent && player.playing}
            <PlayingIndicator />
        {:else}
            <span
                class={isCurrent
                    ? 'font-bold themed-text-accent'
                    : 'themed-text-secondary'}
            >
                {index + 1}
            </span>
        {/if}
    </div>
{/snippet}

{#snippet trackCover(track: Track, cover: string | null)}
    <div
        {@attach lazyCover(track)}
        class="w-10 h-10 rounded-md overflow-hidden shrink-0 transition-transform duration-100! group-hover:scale-105 group-hover:shadow-md"
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
{/snippet}

{#snippet trackTitle(track: Track, cover: string | null, isCurrent: boolean)}
    <div class="flex items-center gap-4 pl-3 overflow-hidden z-10 min-w-0">
        {@render trackCover(track, cover)}

        <div class="flex flex-col min-w-0">
            <span
                class={[
                    'text-sm font-medium truncate',
                    isCurrent ? 'themed-text-accent' : 'themed-text-primary',
                ]}
            >
                {track.title}
            </span>

            <span class="text-xs truncate themed-text-secondary mt-0.5">
                {track.artist}
            </span>
        </div>
    </div>
{/snippet}

{#snippet trackAlbum(track: Track)}
    <div
        class="text-sm truncate hidden md:block z-10 themed-text-secondary min-w-0"
    >
        {track.album}
    </div>
{/snippet}

{#snippet trackTags(track: Track)}
    {@const tags = getTrackTags(track)}

    <div class="flex flex-wrap items-center gap-1 z-10 min-w-0">
        {#if tags.length > 0}
            {#each tags as tag (tag)}
                <span
                    class="themed-tag-chip rounded-full border px-2 py-0.5 text-xs leading-none"
                >
                    {tag}
                </span>
            {/each}
        {:else}
            <span class="block min-h-5"></span>
        {/if}
    </div>
{/snippet}

{#snippet trackDuration(track: Track)}
    <div class="text-sm font-mono z-10 themed-text-secondary text-left">
        {formatDuration(track.duration)}
    </div>
{/snippet}

{#snippet trackItem(track: Track, index: number)}
    {@const isCurrent = player.currentTrack?.id === track.id}
    {@const cover = trackCovers.get(track)}

    <mdui-list-item
        class={[
            'group w-full rounded-xl transition-all duration-200 text-left overflow-hidden outline-none active:scale-[0.995] themed-item',
            isCurrent && 'themed-item-active',
        ]}
        ondblclick={() => handlePlay(track)}
        role="button"
        tabindex="0"
    >
        <mdui-ripple></mdui-ripple>

        <div
            class="grid items-center gap-4 w-full"
            style:grid-template-columns={gridTemplate}
        >
            {#if columns.includes('index')}
                {@render trackIndex(index, isCurrent)}
            {/if}

            {#if columns.includes('title')}
                {@render trackTitle(track, cover, isCurrent)}
            {/if}

            {#if columns.includes('album')}
                {@render trackAlbum(track)}
            {/if}

            {#if columns.includes('tags')}
                {@render trackTags(track)}
            {/if}

            {#if columns.includes('duration')}
                {@render trackDuration(track)}
            {/if}
        </div>
    </mdui-list-item>
{/snippet}

<mdui-list class="flex flex-col w-full">
    <mdui-list-subheader
        class="sticky grid items-center gap-4 w-full pl-4 pr-6 text-xs uppercase tracking-wider font-semibold themed-text-secondary themed-border-b"
        style:grid-template-columns={gridTemplate}
        noninteractive
    >
        {#if columns.includes('index')}
            <div class="text-center">#</div>
        {/if}

        {#if columns.includes('title')}
            <div class="pl-2 text-left">标题</div>
        {/if}

        {#if columns.includes('album')}
            <div class="text-left hidden md:block">专辑</div>
        {/if}

        {#if columns.includes('tags')}
            <div class="text-left">标签</div>
        {/if}

        {#if columns.includes('duration')}
            <div class="text-left">时长</div>
        {/if}
    </mdui-list-subheader>

    <div class="flex flex-col gap-1 mt-2">
        {#each tracks as track, index (track.path)}
            {@render trackItem(track, index)}
        {/each}
    </div>
</mdui-list>

<style>
    .themed-text-primary {
        color: rgb(var(--mdui-color-on-surface));
    }

    .themed-text-secondary {
        color: rgb(var(--mdui-color-on-surface-variant));
    }

    .themed-text-accent {
        color: rgb(var(--mdui-color-primary));
    }

    .themed-border-b {
        border-bottom: 1px solid rgb(var(--mdui-color-outline-variant));
    }

    .themed-surface-high {
        background-color: rgb(var(--mdui-color-surface-container-high));
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

    .themed-tag-chip {
        color: rgb(var(--mdui-color-on-surface-variant));
        background-color: rgb(var(--mdui-color-surface-container-high));
        border-color: rgb(var(--mdui-color-outline-variant));
    }
</style>
