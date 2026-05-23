<script lang="ts">
    import TrackList from '$lib/components/TrackList.svelte'
    import { musicLibrary } from '$lib/library.svelte'
    import { player } from '$lib/player.svelte'
    import 'mdui/components/list-item.js'
    import 'mdui/components/list-subheader.js'
    import 'mdui/components/list.js'
    import 'mdui/components/ripple.js'
    import { onMount } from 'svelte'

    let didInit = false

    onMount(() => {
        if (didInit) return
        didInit = true

        void initLibraryPage()
    })

    async function initLibraryPage() {
        try {
            console.trace('[library/+page.svelte] onMount init')

            const tracks = await musicLibrary.refresh()

            if (player.playlist.length === 0 && tracks.length > 0) {
                player.playlist = tracks

                if (player.currentIndex === -1) {
                    player.currentIndex = 0
                }
            }
        } catch (err) {
            console.error('加载本地媒体库失败:', err)
        }
    }
</script>

<section class="w-full min-h-full overflow-y-auto">
    {#if musicLibrary.tracks.length !== 0}
        <header
            class="flex justify-between items-end pb-3 border-b themed-border"
        >
            <h2 class="text-2xl font-bold tracking-wide themed-text-primary">
                我的音乐库
            </h2>
            <span class="text-sm themed-text-secondary"
                >{musicLibrary.tracks.length} 首歌曲</span
            >
        </header>
        <TrackList tracks={musicLibrary.tracks} />
    {:else}
        <div class="flex h-full w-full justify-center items-center">
            {#if musicLibrary.isLoading}
                正在加载媒体库...
            {:else}
                没有音乐哦
            {/if}
        </div>
    {/if}
</section>
