<script lang="ts">
    import PlayerBar from '$lib/features/PlayerBar.svelte'
    import Appbar from '$lib/features/shell/Appbar.svelte'
    import NavRail from '$lib/features/shell/NavRail.svelte'
    import 'mdui'
    import 'mdui/mdui.css'
    import '../app.css'

    let { children } = $props()
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
