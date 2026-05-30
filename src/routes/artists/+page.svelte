<script lang="ts">
    import Button from '$lib/components/base/Button.svelte'
    import Heading from '$lib/components/base/Heading.svelte'
    import IconButton from '$lib/components/base/IconButton.svelte'
    import SearchBar from '$lib/components/base/SearchBar.svelte'
    import TrackList from '$lib/features/TrackList.svelte'
    import { musicLibrary } from '$lib/state/library.svelte'
    import { player } from '$lib/state/player.svelte'
    import type { Artist, Track } from '$lib/types'

    import 'mdui/components/circular-progress.js'
    import 'mdui/components/list-item.js'
    import 'mdui/components/list.js'

    let searchQuery = $state('')
    let selectedArtist = $state<Artist | null>(null)
    let windowWidth = $state(1024)

    const isMobile = $derived(windowWidth < 768)

    const collator = new Intl.Collator('zh-Hans-CN', {
        numeric: true,
        sensitivity: 'base',
    })

    function createArtistId(name: string) {
        return encodeURIComponent(name)
    }

    function buildArtists(tracks: Track[]): Artist[] {
        const artistMap = new Map<
            string,
            {
                name: string
                tracks: Track[]
                albums: Set<string>
                cover?: string | null
            }
        >()

        for (const track of tracks) {
            const name = track.artist?.trim() || '未知歌手'
            const key = name.toLowerCase()

            if (!artistMap.has(key)) {
                artistMap.set(key, {
                    name,
                    tracks: [],
                    albums: new Set(),
                    cover: track.cover ?? null,
                })
            }

            const artist = artistMap.get(key)!
            artist.tracks.push(track)

            if (track.album?.trim()) {
                artist.albums.add(track.album.trim())
            }

            if (!artist.cover && track.cover) {
                artist.cover = track.cover
            }
        }

        return Array.from(artistMap.values()).map(artist => ({
            id: createArtistId(artist.name),
            name: artist.name,
            cover: artist.cover,
            trackCount: artist.tracks.length,
            albumCount: artist.albums.size,
            tracks: artist.tracks,
        }))
    }

    const artists = $derived(buildArtists(musicLibrary.tracks))

    const filteredArtists = $derived.by(() => {
        const keyword = searchQuery.trim().toLowerCase()
        const list = keyword
            ? artists.filter(artist =>
                  artist.name.toLowerCase().includes(keyword),
              )
            : artists

        return [...list].sort((a, b) => collator.compare(a.name, b.name))
    })

    function playAll(tracks: Track[]) {
        if (tracks.length === 0) return
        player.replacePlaylistAndPlay(tracks, tracks[0].id)
    }
</script>

<svelte:window bind:innerWidth={windowWidth} />

<svelte:head>
    <title>艺术家</title>
</svelte:head>

<div class="flex flex-col h-full overflow-hidden">
    <header
        class="pb-4 shrink-0 border-b border-[rgb(var(--mdui-color-outline-variant))]"
    >
        <Heading eyebrow="Artist" title="歌手" />
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
            {#if selectedArtist === null}
                <div class="flex-1 overflow-y-auto pb-8">
                    {#if filteredArtists.length === 0}
                        <div
                            class="flex flex-col items-center justify-center h-64 text-[rgb(var(--mdui-color-on-surface-variant))]"
                        >
                            <span class="text-4xl mb-2">👤</span>
                            <span class="text-sm">没有找到歌手</span>
                        </div>
                    {:else}
                        <mdui-list>
                            {#each filteredArtists as artist (artist.id)}
                                <mdui-list-item
                                    headline={artist.name}
                                    description={`${artist.trackCount} 首歌曲 · ${artist.albumCount} 张专辑`}
                                    onclick={() => {
                                        selectedArtist = artist
                                    }}
                                    onkeydown={(e: KeyboardEvent) =>
                                        (e.key === 'Enter' || e.key === ' ') &&
                                        (selectedArtist = artist)}
                                    role="button"
                                    tabindex="0"
                                >
                                    {#if artist.cover}
                                        <img
                                            src={artist.cover}
                                            slot="icon"
                                            class="w-10 h-10 rounded-full object-cover shrink-0"
                                            alt=""
                                        />
                                    {:else}
                                        <div
                                            slot="icon"
                                            class="w-10 h-10 rounded-full bg-[rgb(var(--mdui-color-surface-container-highest))] flex items-center justify-center text-lg shrink-0"
                                        >
                                            👤
                                        </div>
                                    {/if}
                                </mdui-list-item>
                            {/each}
                        </mdui-list>
                    {/if}
                </div>
            {:else}
                <div class="flex-1 flex flex-col overflow-hidden min-h-0">
                    <div
                        class="flex items-center gap-3 py-3 border-b border-[rgb(var(--mdui-color-outline-variant))] shrink-0"
                    >
                        <IconButton
                            icon="arrow_back--rounded"
                            onclick={() => {
                                selectedArtist = null
                            }}
                        />
                        <div class="flex-1 min-w-0">
                            <h2
                                class="text-base font-semibold truncate text-[rgb(var(--mdui-color-on-surface))]"
                            >
                                {selectedArtist.name}
                            </h2>
                            <p
                                class="text-xs text-[rgb(var(--mdui-color-on-surface-variant))]"
                            >
                                {selectedArtist.trackCount} 首歌曲 · {selectedArtist.albumCount}
                                张专辑
                            </p>
                        </div>
                        <Button
                            variant="filled"
                            onclick={() => playAll(selectedArtist!.tracks)}
                            >播放全部</Button
                        >
                    </div>
                    <div class="flex-1 overflow-y-auto pb-8">
                        <TrackList tracks={selectedArtist.tracks} />
                    </div>
                </div>
            {/if}
        </div>
    {:else}
        <div class="flex-1 flex overflow-hidden min-h-0">
            <div
                class="flex-1 shrink-0 border-r border-[rgb(var(--mdui-color-outline-variant))] overflow-y-auto pb-8"
            >
                {#if filteredArtists.length === 0}
                    <div
                        class="flex flex-col items-center justify-center h-64 text-[rgb(var(--mdui-color-on-surface-variant))]"
                    >
                        <span class="text-4xl mb-2">👤</span>
                        <span class="text-sm">没有找到歌手</span>
                    </div>
                {:else}
                    <mdui-list>
                        {#each filteredArtists as artist (artist.id)}
                            <mdui-list-item
                                headline={artist.name}
                                description={`${artist.trackCount} 首歌曲 · ${artist.albumCount} 张专辑`}
                                onclick={() => {
                                    selectedArtist = artist
                                }}
                                onkeydown={(e: KeyboardEvent) =>
                                    (e.key === 'Enter' || e.key === ' ') &&
                                    (selectedArtist = artist)}
                                role="button"
                                tabindex="0"
                                active={selectedArtist?.id === artist.id}
                            >
                                {#if artist.cover}
                                    <img
                                        src={artist.cover}
                                        slot="icon"
                                        class="w-10 h-10 rounded-full object-cover shrink-0"
                                        alt=""
                                    />
                                {:else}
                                    <div
                                        slot="icon"
                                        class="w-10 h-10 rounded-full bg-[rgb(var(--mdui-color-surface-container-highest))] flex items-center justify-center text-lg shrink-0"
                                    >
                                        👤
                                    </div>
                                {/if}
                            </mdui-list-item>
                        {/each}
                    </mdui-list>
                {/if}
            </div>
            <div class="flex-1 flex flex-col overflow-hidden min-h-0">
                {#if selectedArtist === null}
                    <div
                        class="flex-1 flex flex-col items-center justify-center text-[rgb(var(--mdui-color-on-surface-variant))]"
                    >
                        <span class="text-5xl mb-2">👤</span>
                        <span class="text-sm">选择一个歌手以查看歌曲</span>
                    </div>
                {:else}
                    <div
                        class="flex items-center justify-between p-4 border-b border-[rgb(var(--mdui-color-outline-variant))] shrink-0"
                    >
                        <div>
                            <h2
                                class="text-lg font-semibold text-[rgb(var(--mdui-color-on-surface))]"
                            >
                                {selectedArtist.name}
                            </h2>
                            <p
                                class="text-xs text-[rgb(var(--mdui-color-on-surface-variant))]"
                            >
                                {selectedArtist.trackCount} 首歌曲 · {selectedArtist.albumCount}
                                张专辑
                            </p>
                        </div>
                        <Button
                            variant="filled"
                            onclick={() => playAll(selectedArtist!.tracks)}
                            >播放全部</Button
                        >
                    </div>
                    <div class="flex-1 overflow-y-auto pb-8">
                        <TrackList tracks={selectedArtist.tracks} columns={['index', 'title', 'duration']}/>
                    </div>
                {/if}
            </div>
        </div>
    {/if}
</div>
