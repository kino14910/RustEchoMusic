<script lang="ts">
    import PlayerBar from '$lib/features/PlayerBar.svelte'
    import Appbar from '$lib/features/shell/Appbar.svelte'
    import NavRail from '$lib/features/shell/NavRail.svelte'
    import { settings } from '$lib/state/settings.svelte'
    import 'mdui'
    import 'mdui/mdui.css'
    import { onMount } from 'svelte'
    import '../app.css'

    let { children } = $props()

    onMount(() => {
        void settings.load()
    })

    $effect(() => {
        document.documentElement.style.setProperty(
            '--transition-duration',
            settings.data.reduceMotion ? '0s' : '0.2s',
        )
    })
</script>

<mdui-layout full-height>
    <Appbar />
    <PlayerBar />
    <NavRail />

    <mdui-layout-main
        class="flex flex-col h-screen w-screen overflow-hidden bg-(--controlWhite) text-(--controlBlack)"
    >
        {@render children()}
    </mdui-layout-main>
</mdui-layout>

<style lang="postcss">
    @reference "tailwindcss";
    @layer base {
        :global(*) {
            transition: all var(--transition-duration, 0.2s) ease;
        }
    }
</style>
