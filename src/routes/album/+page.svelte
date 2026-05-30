<script lang="ts">
    import Button from '$lib/components/base/Button.svelte'
    import Heading from '$lib/components/base/Heading.svelte'
    import IconButton from '$lib/components/base/IconButton.svelte'
    import SearchBar from '$lib/components/base/SearchBar.svelte'
    import TrackList from '$lib/features/TrackList.svelte'
    import { trackCovers } from '$lib/state/covers.svelte'
    import { musicLibrary } from '$lib/state/library.svelte'
    import { player } from '$lib/state/player.svelte'
    import type { Album, Track } from '$lib/types'

    import 'mdui/components/circular-progress.js'
    import 'mdui/components/list-item.js'
    import 'mdui/components/list.js'

    let searchQuery = $state('')
    let selectedAlbum = $state<Album | null>(null)
    let windowWidth = $state(1024)

    const isMobile = $derived(windowWidth < 768)

    const collator = new Intl.Collator('zh-Hans-CN', {
        numeric: true,
        sensitivity: 'base',
    })

    function buildAlbums(tracks: Track[]): Album[] {
        const albumMap = new Map<
            string,
            {
                title: string
                tracks: Track[]
                cover?: string | null
                representativeTrack: Track
            }
        >()

        for (const track of tracks) {
            const title = track.album?.trim() || '未知专辑'
            const key = title.toLowerCase()

            if (!albumMap.has(key)) {
                albumMap.set(key, {
                    title,
                    tracks: [],
                    cover: track.cover ?? null,
                    representativeTrack: track,
                })
            }

            const album = albumMap.get(key)!
            album.tracks.push(track)

            if (!album.cover && track.cover) {
                album.cover = track.cover
            }
        }

        return Array.from(albumMap.values()).map(album => ({
            ...album,
            id: encodeURIComponent(album.title),
            trackCount: album.tracks.length,
        }))
    }

    const albums = $derived(buildAlbums(musicLibrary.tracks))

    const filteredAlbums = $derived.by(() => {
        const keyword = searchQuery.trim().toLowerCase()
        const list = keyword
            ? albums.filter(album =>
                  album.title.toLowerCase().includes(keyword),
              )
            : albums

        return [...list].sort((a, b) => collator.compare(a.title, b.title))
    })

    $effect(() => {
        for (const album of filteredAlbums) {
            void trackCovers.load(album.representativeTrack)
        }
    })

    function playAll(tracks: Track[]) {
        if (tracks.length === 0) return
        player.replacePlaylistAndPlay(tracks, tracks[0].id)
    }
</script>

<svelte:window bind:innerWidth={windowWidth} />

<svelte:head>
    <title>专辑</title>
</svelte:head>

<div class="flex flex-col h-full overflow-hidden">
    <header class="pb-4 shrink-0 border-b border-[rgb(var(--mdui-color-outline-variant))]">
        <Heading eyebrow="Albums" title="专辑" />
        <div class="mt-2 px-1 max-w-md">
            <SearchBar bind:value={searchQuery} />
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
    {:else if isMobile}
        <div class="flex-1 overflow-hidden min-h-0 flex flex-col">
            {#if selectedAlbum === null}
                <div class="flex-1 overflow-y-auto pb-8">
                    {#if filteredAlbums.length === 0}
                        <div class="flex flex-col items-center justify-center h-64 text-[rgb(var(--mdui-color-on-surface-variant))]">
                            <span class="text-4xl mb-2">🎵</span>
                            <span class="text-sm">没有找到专辑</span>
                        </div>
                    {:else}
                        <mdui-list>
                            {#each filteredAlbums as album (album.id)}
                                {@const cover = trackCovers.get(album.representativeTrack) ?? album.cover}
                                <mdui-list-item
                                    headline={album.title}
                                    description={`${album.trackCount} 首歌曲`}
                                    onclick={() => { selectedAlbum = album }}
                                    onkeydown={(e: KeyboardEvent) =>
                                        (e.key === 'Enter' || e.key === ' ') && (selectedAlbum = album)}
                                    role="button"
                                    tabindex="0"
                                >
                                    {#if cover}
                                        <img src={cover} slot="icon" class="w-10 h-10 rounded object-cover shrink-0" alt="" />
                                    {:else}
                                        <div slot="icon" class="w-10 h-10 rounded bg-[rgb(var(--mdui-color-surface-container-highest))] flex items-center justify-center text-lg shrink-0">
                                            🎵
                                        </div>
                                    {/if}
                                </mdui-list-item>
                            {/each}
                        </mdui-list>
                    {/if}
                </div>
            {:else}
                <div class="flex-1 flex flex-col overflow-hidden min-h-0">
                    <div class="flex items-center gap-3 py-3 border-b border-[rgb(var(--mdui-color-outline-variant))] shrink-0">
                        <IconButton icon="arrow_back--rounded" onclick={() => { selectedAlbum = null }} />
                        <div class="flex-1 min-w-0">
                            <h2 class="text-base font-semibold truncate text-[rgb(var(--mdui-color-on-surface))]">{selectedAlbum.title}</h2>
                            <p class="text-xs text-[rgb(var(--mdui-color-on-surface-variant))]">{selectedAlbum.trackCount} 首歌曲</p>
                        </div>
                        <Button variant="filled" onclick={() => playAll(selectedAlbum!.tracks)}>播放全部</Button>
                    </div>
                    <div class="flex-1 overflow-y-auto pb-8">
                        <TrackList tracks={selectedAlbum.tracks} />
                    </div>
                </div>
            {/if}
        </div>
    {:else}
        <div class="flex-1 flex overflow-hidden min-h-0">
            <div class="flex-1 shrink-0 border-r border-[rgb(var(--mdui-color-outline-variant))] overflow-y-auto pb-8">
                {#if filteredAlbums.length === 0}
                    <div class="flex flex-col items-center justify-center h-64 text-[rgb(var(--mdui-color-on-surface-variant))]">
                        <span class="text-4xl mb-2">🎵</span>
                        <span class="text-sm">没有找到专辑</span>
                    </div>
                {:else}
                    <mdui-list>
                        {#each filteredAlbums as album (album.id)}
                            {@const cover = trackCovers.get(album.representativeTrack) ?? album.cover}
                            <mdui-list-item
                                headline={album.title}
                                description={`${album.trackCount} 首歌曲`}
                                onclick={() => { selectedAlbum = album }}
                                onkeydown={(e: KeyboardEvent) =>
                                    (e.key === 'Enter' || e.key === ' ') && (selectedAlbum = album)}
                                role="button"
                                tabindex="0"
                                active={selectedAlbum?.id === album.id}
                            >
                                {#if cover}
                                    <img src={cover} slot="icon" class="w-10 h-10 rounded object-cover shrink-0" alt="" />
                                {:else}
                                    <div slot="icon" class="w-10 h-10 rounded bg-[rgb(var(--mdui-color-surface-container-highest))] flex items-center justify-center text-lg shrink-0">
                                        🎵
                                    </div>
                                {/if}
                            </mdui-list-item>
                        {/each}
                    </mdui-list>
                {/if}
            </div>
            <div class="flex-1 flex flex-col overflow-hidden min-h-0">
                {#if selectedAlbum === null}
                    <div class="flex-1 flex flex-col items-center justify-center text-[rgb(var(--mdui-color-on-surface-variant))]">
                        <span class="text-5xl mb-2">🎵</span>
                        <span class="text-sm">选择一个专辑以查看歌曲</span>
                    </div>
                {:else}
                    <div class="flex items-center justify-between p-4 border-b border-[rgb(var(--mdui-color-outline-variant))] shrink-0">
                        <div>
                            <h2 class="text-lg font-semibold text-[rgb(var(--mdui-color-on-surface))]">{selectedAlbum.title}</h2>
                            <p class="text-xs text-[rgb(var(--mdui-color-on-surface-variant))]">{selectedAlbum.trackCount} 首歌曲</p>
                        </div>
                        <Button variant="filled" onclick={() => playAll(selectedAlbum!.tracks)}>播放全部</Button>
                    </div>
                    <div class="flex-1 overflow-y-auto pb-8">
                        <TrackList tracks={selectedAlbum.tracks} columns={['index', 'title', 'duration']}/>
                    </div>
                {/if}
            </div>
        </div>
    {/if}
</div>
