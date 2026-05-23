<script lang="ts">
    import TrackList from '$lib/components/TrackList.svelte'
    import { recentlyPlayed } from '$lib/recent.svelte'
    import { onMount } from 'svelte'

    import 'mdui/components/button.js'
    import 'mdui/components/circular-progress.js'

    onMount(() => {
        void recentlyPlayed.load()
    })
</script>

<svelte:head>
    <title>最近播放</title>
</svelte:head>

<section class="w-full min-h-full overflow-y-auto">
    <header class="flex items-end justify-between gap-4 shrink-0 pb-3 border-b themed-border">
        <div class="min-w-0">
            <h1 class="text-2xl font-bold themed-text-primary">最近播放</h1>
            <p class="text-sm themed-text-secondary mt-1">
                这里会显示你最近播放过的歌曲
            </p>
        </div>

        {#if recentlyPlayed.tracks.length > 0}
            <mdui-button
                variant="outlined"
                onclick={() => recentlyPlayed.clear()}
                onkeydown={(e: KeyboardEvent) => {
                    if (e.key === 'Enter' || e.key === ' ') recentlyPlayed.clear()
                }}
                role="button"
                tabindex="0"
            >
                清空
            </mdui-button>
        {/if}
    </header>

    {#if recentlyPlayed.isLoading}
        <div class="flex-1 flex items-center justify-center">
            <mdui-circular-progress></mdui-circular-progress>
        </div>
    {:else if recentlyPlayed.error}
        <div class="flex-1 flex items-center justify-center text-red-500">
            {recentlyPlayed.error}
        </div>
    {:else if recentlyPlayed.tracks.length === 0}
        <div
            class="flex-1 flex flex-col items-center justify-center gap-2 themed-text-secondary"
        >
            <div class="text-5xl">🎧</div>
            <div class="text-base font-medium">还没有最近播放记录</div>
            <div class="text-sm">播放一首歌后，它会出现在这里</div>
        </div>
    {:else}
        <div class="flex-1 overflow-auto">
            <TrackList tracks={recentlyPlayed.tracks} />
        </div>
    {/if}
</section>