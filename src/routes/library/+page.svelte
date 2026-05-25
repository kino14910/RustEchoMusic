<script lang="ts">
    import TrackList from '$lib/features/TrackList.svelte'
    import { musicLibrary } from '$lib/state/library.svelte'
    import { player } from '$lib/state/player.svelte'
    import type { Track } from '$lib/types'
    import { onMount } from 'svelte'

    import Button from '$lib/components/base/Button.svelte'
    import Heading from '$lib/components/base/Heading.svelte'
    import Filters from '$lib/features/Filters.svelte'
    import 'mdui/components/button.js'
    import 'mdui/components/circular-progress.js'

    type SortBy = 'title' | 'artist' | 'album'

    let query = $state('')
    let sortBy = $state<SortBy>('title')

    const collator = new Intl.Collator('zh-Hans-CN', {
        numeric: true,
        sensitivity: 'base',
    })

    onMount(() => {
        void musicLibrary.refresh()
    })

    let filteredTracks = $derived.by(() => {
        const keyword = query.trim().toLowerCase()

        const tracks = keyword
            ? musicLibrary.tracks.filter(track =>
                  [track.title, track.artist, track.album, track.path]
                      .filter(Boolean)
                      .some(value => value.toLowerCase().includes(keyword)),
              )
            : musicLibrary.tracks

        return [...tracks].sort((a, b) => {
            return collator.compare(a[sortBy] ?? '', b[sortBy] ?? '')
        })
    })

    let totalDuration = $derived(
        musicLibrary.tracks.reduce((sum, track) => sum + track.duration, 0),
    )

    function playAll() {
        if (filteredTracks.length === 0) return

        player.playlist = filteredTracks
        void player.playByIndex(0)
    }
</script>

<svelte:head>
    <title>歌曲</title>
</svelte:head>

<header
    class="flex flex-col gap-4 border-b border-[rgb(var(--mdui-color-outline-variant))] pb-5"
>
    <Heading eyebrow="library" title="歌曲">
        <div class="flex items-center gap-2">
            <Button
                variant="filled"
                disabled={filteredTracks.length === 0}
                onclick={playAll}
            >
                播放全部
            </Button>

            <Button
                variant="outlined"
                onclick={() => musicLibrary.refresh({ force: true })}
            >
                刷新
            </Button>
        </div>
    </Heading>
    <Filters
        bind:query
        bind:sortBy
        searchPlaceholder="搜索标题、歌手、专辑..."
        sortOptions={[
            { label: '按标题排序', value: 'title' },
            { label: '按歌手排序', value: 'artist' },
            { label: '按专辑排序', value: 'album' },
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
{:else if filteredTracks.length === 0}
    <div
        class="flex flex-1 flex-col items-center justify-center gap-2 text-[rgb(var(--mdui-color-on-surface-variant))]"
    >
        <div class="text-5xl">🎵</div>
        <div class="text-base font-medium">没有找到歌曲</div>
        <div class="text-sm">尝试刷新媒体库或修改搜索关键词</div>
    </div>
{:else}
    <div class="min-h-0 flex-1 overflow-auto">
        <TrackList tracks={filteredTracks as Track[]} />
    </div>
{/if}
