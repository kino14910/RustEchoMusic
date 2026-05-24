<script lang="ts">
    import MediaGrid from '$lib/components/media/MediaGrid.svelte'
    import { trackCovers } from '$lib/state/covers.svelte'
    import { musicLibrary } from '$lib/state/library.svelte'
    import type { Album, Track } from '$lib/types'
    import { onMount } from 'svelte'

    import Button from '$lib/components/base/Button.svelte'
    import Filters from '$lib/features/Filters.svelte'
    import 'mdui/components/button.js'
    import 'mdui/components/circular-progress.js'
    import 'mdui/components/menu-item.js'
    import 'mdui/components/select.js'
    import 'mdui/components/text-field.js'
    type SortBy = 'title' | 'artist' | 'trackCount'

    let query = $state('')
    let sortBy = $state<SortBy>('title')

    const collator = new Intl.Collator('zh-Hans-CN', {
        numeric: true,
        sensitivity: 'base',
    })

    onMount(() => {
        void musicLibrary.refresh()
    })

    function createAlbumId(title: string, artist: string) {
        return encodeURIComponent(`${artist}__${title}`)
    }

    function getAlbumTitle(track: Track) {
        return track.album?.trim() || '未知专辑'
    }

    function getAlbumArtist(track: Track) {
        return track.artist?.trim() || '未知艺术家'
    }

    function buildAlbums(tracks: Track[]): Album[] {
        const albumMap = new Map<
            string,
            {
                title: string
                artist: string
                tracks: Track[]
                duration: number
                cover?: string | null
                representativeTrack: Track
            }
        >()

        for (const track of tracks) {
            const title = getAlbumTitle(track)
            const artist = getAlbumArtist(track)
            const key = `${artist.toLowerCase()}__${title.toLowerCase()}`

            if (!albumMap.has(key)) {
                albumMap.set(key, {
                    title,
                    artist,
                    tracks: [],
                    duration: 0,
                    cover: track.cover ?? null,
                    representativeTrack: track,
                })
            }

            const album = albumMap.get(key)!

            album.tracks.push(track)
            album.duration += track.duration

            if (!album.cover && track.cover) {
                album.cover = track.cover
            }
        }

        return Array.from(albumMap.values()).map(album => ({
            id: createAlbumId(album.title, album.artist),
            title: album.title,
            artist: album.artist,
            cover: album.cover,
            trackCount: album.tracks.length,
            duration: album.duration,
            tracks: album.tracks,
            representativeTrack: album.representativeTrack,
        }))
    }

    let albums = $derived.by(() => buildAlbums(musicLibrary.tracks))

    let filteredAlbums = $derived.by(() => {
        const keyword = query.trim().toLowerCase()

        const list = keyword
            ? albums.filter(album =>
                  [album.title, album.artist].some(value =>
                      value.toLowerCase().includes(keyword),
                  ),
              )
            : albums

        return [...list].sort((a, b) => {
            if (sortBy === 'title') {
                return collator.compare(a.title, b.title)
            }

            if (sortBy === 'artist') {
                return collator.compare(a.artist, b.artist)
            }

            return b[sortBy] - a[sortBy]
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
            subtitle: `${album.artist} · ${album.trackCount} 首歌曲`,
            image: trackCovers.get(album.representativeTrack) ?? album.cover,
            href: `/albums/${album.id}`,
            shape: 'square' as const,
        })),
    )
</script>

<svelte:head>
    <title>专辑</title>
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
                    Albums
                </p>

                <h1
                    class="mt-1 text-3xl font-bold tracking-tight text-[rgb(var(--mdui-color-on-surface))]"
                >
                    专辑
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
            searchPlaceholder="搜索专辑或歌手..."
            sortOptions={[
                    { label: '按专辑名排序', value: 'title' },
                    { label: '按歌手排序', value: 'artist' },
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
</section>
