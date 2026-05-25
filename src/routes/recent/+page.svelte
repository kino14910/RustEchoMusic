<script lang="ts">
    import TrackList from '$lib/features/TrackList.svelte'
    import { recentlyPlayed } from '$lib/state/recent.svelte'
    import { onMount } from 'svelte'

    import Button from '$lib/components/base/Button.svelte'
    import Heading from '$lib/components/base/Heading.svelte'
    import 'mdui/components/button.js'
    import 'mdui/components/circular-progress.js'

    onMount(() => {
        void recentlyPlayed.load()
    })
</script>

<svelte:head>
    <title>最近播放</title>
</svelte:head>

<header class="border-b border-[rgb(var(--mdui-color-outline-variant))] pb-5">
    <Heading eyebrow="Recent" title="最近播放">
        {#if recentlyPlayed.tracks.length > 0}
            <Button variant="outlined" onclick={() => recentlyPlayed.clear()}>
                清空
            </Button>
        {/if}
    </Heading>
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
