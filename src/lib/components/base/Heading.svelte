<script lang="ts">
    import type { Snippet } from 'svelte'

    type PageHeadingProps = {
        eyebrow: string
        title: string
        children: Snippet
        class?: string
    }

    let {
        eyebrow,
        title,
        children,
        class: className = '',
    }: PageHeadingProps = $props()
</script>

<div class="flex flex-wrap items-end justify-between gap-4">
    <div class="min-w-0">
        <p
            class="text-xs font-semibold uppercase tracking-wider text-[rgb(var(--mdui-color-primary))]"
        >
            {eyebrow}
        </p>

        <h1
            class="mt-1 text-3xl font-bold tracking-tight text-[rgb(var(--mdui-color-on-surface))]"
        >
            {title}
        </h1>
    </div>

    {@render children()}
</div>
