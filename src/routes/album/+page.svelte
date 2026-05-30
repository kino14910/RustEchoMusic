<script lang="ts">
    import Button from '$lib/components/base/Button.svelte'
    import Heading from '$lib/components/base/Heading.svelte'
    import MediaGrid from '$lib/components/media/MediaGrid.svelte'
    import Filters from '$lib/features/Filters.svelte'
    import { trackCovers } from '$lib/state/covers.svelte'
    import { musicLibrary } from '$lib/state/library.svelte'
    import type { Album, Track } from '$lib/types'

    import 'mdui/components/circular-progress.js'

    type SortBy = 'title' | 'trackCount'

    let query = $state('')
    let sortBy = $state<SortBy>('title')

    const collator = new Intl.Collator('zh-Hans-CN', {
        numeric: true,
        sensitivity: 'base',
    })

    function createAlbumId(title: string) {
        return encodeURIComponent(title)
    }

    function getAlbumTitle(track: Track) {
        return track.album?.trim() || '未知专辑'
    }

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
            const title = getAlbumTitle(track)
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
            id: createAlbumId(album.title),
            trackCount: album.tracks.length,
        }))
    }

    let albums = $derived.by(() => buildAlbums(musicLibrary.tracks))

    let filteredAlbums = $derived.by(() => {
        const keyword = query.trim().toLowerCase()

        const list = keyword
            ? albums.filter(album =>
                  album.title.toLowerCase().includes(keyword),
              )
            : albums

        return [...list].sort((a, b) => {
            if (sortBy === 'title') {
                return collator.compare(a.title, b.title)
            }

            return b.trackCount - a.trackCount
        })
    })

    $effect(() => {
        for (const album of filteredAlbums) {
            void trackCovers.load(album.representativeTrack)
        }
    })

    let albumItems = $derived(
        filteredAlbums.map(album => ({
            id: album.id,
            title: album.title,
            subtitle: `${album.trackCount} 首歌曲`,
            image: trackCovers.get(album.representativeTrack) ?? album.cover,
            href: `/albums/${album.id}`,
            shape: 'square' as const,
        })),
    )
</script>

<svelte:head>
    <title>专辑</title>
</svelte:head>

<header class="pb-5">
    <Heading eyebrow="Albums" title="专辑">
        <Button variant="outlined" onclick={() => musicLibrary.scan()}>
            刷新
        </Button>
    </Heading>

    <Filters
        bind:query
        bind:sortBy
        searchPlaceholder="搜索专辑..."
        sortOptions={[
            { label: '按专辑名排序', value: 'title' },
            { label: '按歌曲数排序', value: 'trackCount' },
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
            items={albumItems}
            emptyTitle="没有找到专辑"
            emptyDescription="尝试刷新媒体库或修改搜索关键词"
        />
    </div>
{/if}
