<script lang="ts">
    import { goto } from '$app/navigation'
    import { onMount } from 'svelte'

    onMount(() => {
        goto('/library', { replaceState: true })
    })
</script>

<style lang="postcss">
    @reference "tailwindcss";
    @layer base {
        :global(html) {
            background-color: theme(--color-gray-100);
        }
    }
</style>
