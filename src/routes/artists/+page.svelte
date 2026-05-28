<script lang="ts">
    import MediaGrid from '$lib/components/media/MediaGrid.svelte'
    import { musicLibrary } from '$lib/state/library.svelte'
    import type { Artist, Track } from '$lib/types'

    import Button from '$lib/components/base/Button.svelte'
    import Heading from '$lib/components/base/Heading.svelte'
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
            const name = track.artist?.trim() || '未知艺术家'
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
</script>

<svelte:head>
    <title>艺术家</title>
</svelte:head>

<header class="pb-5">
    <Heading eyebrow="artist" title="艺术家">
        <Button
            variant="outlined"
            onclick={() => musicLibrary.scan()}
        >
            刷新
        </Button>
    </Heading>

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
