<script lang="ts">
    import TrackList from '$lib/features/TrackList.svelte'
    import { musicLibrary } from '$lib/state/library.svelte'
    import { player } from '$lib/state/player.svelte'
    import type { Track } from '$lib/types'

    import Button from '$lib/components/base/Button.svelte'
    import Heading from '$lib/components/base/Heading.svelte'
    import SearchBar from '$lib/components/base/SearchBar.svelte'

    import 'mdui/components/button.js'
    import 'mdui/components/circular-progress.js'
    import 'mdui/components/segmented-button-group.js'
    import 'mdui/components/segmented-button.js'

    type SortBy = 'title' | 'artist' | 'album'

    let searchQuery = $state('')
    let sortBy = $state<SortBy>('title')

    const collator = new Intl.Collator('zh-Hans-CN', {
        numeric: true,
        sensitivity: 'base',
    })

    const searchResults = $derived(
        musicLibrary.tracks.filter(track => 
            (track.title ?? '').toLowerCase().includes(searchQuery.toLowerCase()) ||
            (track.artist ?? '').toLowerCase().includes(searchQuery.toLowerCase()) ||
            (track.album ?? '').toLowerCase().includes(searchQuery.toLowerCase())
        )
    )

    const defaultSortedTracks = $derived.by(() => {
        return [...musicLibrary.tracks].sort((a, b) => {
            return collator.compare(a[sortBy] ?? '', b[sortBy] ?? '')
        })
    })

    const displayTracks = $derived(searchQuery ? searchResults : defaultSortedTracks)

    function playAll() {
        if (displayTracks.length === 0) return
        player.replacePlaylistAndPlay(displayTracks, displayTracks[0].id)
    }

    function handleSortChange(event: Event) {
        const value = (event.currentTarget as HTMLElement & { value: string }).value
        if (value === 'title' || value === 'artist' || value === 'album') {
            sortBy = value
        }
    }
</script>

<svelte:head>
    <title>歌曲</title>
</svelte:head>

<div class="flex flex-col h-full overflow-hidden">
    <header class="border-b border-[rgb(var(--mdui-color-outline-variant))] pb-4 shrink-0">
        <Heading eyebrow="library" title="歌曲">
            <Button
                variant="filled"
                disabled={displayTracks.length === 0}
                onclick={playAll}
            >
                播放全部
            </Button>

            <Button
                variant="outlined"
                onclick={() => musicLibrary.scan()}
            >
                刷新
            </Button>
        </Heading>
        <div class="flex px-2 justify-end gap-16 items-center">
            <div class="max-w-md">
                <SearchBar bind:value={searchQuery} />
            </div>
            <div class="flex">
                <mdui-segmented-button-group
                    selects="single"
                    value={sortBy}
                    onchange={handleSortChange}
                >
                    <mdui-segmented-button value="title">标题</mdui-segmented-button>
                    <mdui-segmented-button value="artist">歌手</mdui-segmented-button>
                    <mdui-segmented-button value="album">专辑</mdui-segmented-button>
                </mdui-segmented-button-group>
            </div>
        </div>
    </header>

    {#if musicLibrary.isLoading}
        <div class="flex flex-1 items-center justify-center">
            <mdui-circular-progress></mdui-circular-progress>
        </div>
    {:else if musicLibrary.error}
        <div class="flex flex-1 items-center justify-center text-red-500">
            {musicLibrary.error}
        </div>
    {:else if displayTracks.length === 0}
        <div class="flex flex-1 flex-col items-center justify-center gap-2 text-[rgb(var(--mdui-color-on-surface-variant))]" >
            <div class="text-5xl">🎵</div>
            <div class="text-base font-medium">没有找到歌曲</div>
            <div class="text-sm">尝试刷新媒体库或修改搜索关键词</div>
        </div>
    {:else}
        <div class="min-h-0 flex-1 overflow-auto pb-8">
            <TrackList tracks={displayTracks as Track[]} />
        </div>
    {/if}
</div>
