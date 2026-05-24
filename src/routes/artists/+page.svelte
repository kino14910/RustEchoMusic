<script lang="ts">
    import MediaGrid from '$lib/components/media/MediaGrid.svelte'
    import { musicLibrary } from '$lib/state/library.svelte'
    import type { Artist, Track } from '$lib/types'
    import { onMount } from 'svelte'

    import Button from '$lib/components/base/Button.svelte'
    import Filters from '$lib/features/Filters.svelte'
    import 'mdui/components/button.js'
    import 'mdui/components/circular-progress.js'

    type SortBy = 'name' | 'trackCount' | 'albumCount'

    let query = $state('')
    let sortBy = $state<SortBy>('name')

    const collator = new Intl.Collator('zh-Hans-CN', {
        numeric: true,
        sensitivity: 'base',
    })

    onMount(() => {
        void musicLibrary.refresh()
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
                duration: number
                cover?: string | null
            }
        >()

        for (const track of tracks) {
            const name = track.artist?.trim() || '未知艺术家'
            const key = name.toLowerCase()

            if (!artistMap.has(key)) {
                artistMap.set(key, {
                    name,
                    tracks: [],
                    albums: new Set(),
                    duration: 0,
                    cover: track.cover ?? null,
                })
            }

            const artist = artistMap.get(key)!

            artist.tracks.push(track)
            artist.duration += track.duration

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
            duration: artist.duration,
            tracks: artist.tracks,
        }))
    }

    let artists = $derived.by(() => buildArtists(musicLibrary.tracks))

    let filteredArtists = $derived.by(() => {
        const keyword = query.trim().toLowerCase()

        const list = keyword
            ? artists.filter(artist =>
                  artist.name.toLowerCase().includes(keyword),
              )
            : artists

        return [...list].sort((a, b) => {
            if (sortBy === 'name') {
                return collator.compare(a.name, b.name)
            }

            return b[sortBy] - a[sortBy]
        })
    })

    let artistItems = $derived(
        filteredArtists.map(artist => ({
            id: artist.id,
            title: artist.name,
            subtitle: `${artist.trackCount} 首歌曲 · ${artist.albumCount} 张专辑`,
            image: artist.cover,
            href: `/artists/${artist.id}`,
            shape: 'circle' as const,
        })),
    )

    let totalDuration = $derived(
        artists.reduce((sum, artist) => sum + artist.duration, 0),
    )
</script>

<svelte:head>
    <title>歌手</title>
</svelte:head>

<section class="flex h-full min-h-0 flex-col gap-6 overflow-hidden">
    <header
        class="flex flex-col gap-4 border-b border-[rgb(var(--mdui-color-outline-variant))] pb-5"
    >
        <div class="flex flex-wrap items-end justify-between gap-4">
            <div class="min-w-0">
                <p
                    class="text-xs font-semibold uppercase tracking-wider text-[rgb(var(--mdui-color-primary))]"
                >
                    Artists
                </p>

                <h1
                    class="mt-1 text-3xl font-bold tracking-tight text-[rgb(var(--mdui-color-on-surface))]"
                >
                    歌手
                </h1>
            </div>

            <Button
                variant="outlined"
                onclick={() => musicLibrary.refresh({ force: true })}
            >
                刷新
            </Button>
        </div>

        <Filters
            bind:query
            bind:sortBy
            searchPlaceholder="搜索歌手..."
            sortOptions={[
                { label: '按名称排序', value: 'name' },
                { label: '按歌曲数排序', value: 'trackCount' },
                { label: '按专辑数排序', value: 'albumCount' },
            ]}
        />
    </header>

    {#if musicLibrary.isLoading}
        <div class="flex flex-1 items-center justify-center">
            <mdui-circular-progress></mdui-circular-progress>
        </div>
    {:else if musicLibrary.error}
        <div class="flex flex-1 items-center justify-center text-red-500">
            {musicLibrary.error}
        </div>
    {:else}
        <div class="min-h-0 flex-1 overflow-auto pb-8">
            <MediaGrid
                items={artistItems}
                emptyTitle="没有找到歌手"
                emptyDescription="尝试刷新媒体库或修改搜索关键词"
            />
        </div>
    {/if}
</section>
