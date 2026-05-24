<script lang="ts">
    const { onclick = () => {}, ...props } = $props()
</script>

<mdui-button-icon
    onclick={() => onclick()}
    onkeydown={(e: KeyboardEvent) =>
        (e.key === 'Enter' || e.key === ' ') && onclick()}
    role="button"
    tabindex="0"
    {...props}
></mdui-button-icon>
