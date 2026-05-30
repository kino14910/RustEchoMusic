<script lang="ts">
    type Props = {
        color?: string
    }
    let { color = 'rgb(var(--mdui-color-primary))' }: Props = $props()
</script>

<div class="flex items-end gap-0.5 h-3.5">
    <span
        class="w-0.5 h-full animate-[bounce_0.8s_infinite_alternate] bg-[{color}]"
    ></span>
    <span
        class="w-0.5 h-full animate-[bounce_0.8s_infinite_alternate_0.2s] bg-[{color}]"
    ></span>
    <span
        class="w-0.5 h-full animate-[bounce_0.8s_infinite_alternate_0.4s] bg-[{color}]"
    ></span>
</div>
