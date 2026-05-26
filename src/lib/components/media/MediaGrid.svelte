<script lang="ts">
    import 'mdui/components/button-icon.js'
    import 'mdui/components/card.js'

    export type MediaGridItem = {
        id: string | number
        title: string
        subtitle?: string
        image?: string | null
        href?: string
        shape?: 'square' | 'circle'
    }

    let {
        items,
        emptyTitle = '暂无内容',
        emptyDescription = '这里还没有可以显示的内容',
    }: {
        items: MediaGridItem[]
        emptyTitle?: string
        emptyDescription?: string
    } = $props()
</script>

{#snippet mediaCard(item: MediaGridItem)}
    <mdui-card
        href={item.href ?? undefined}
        class="group"
        aria-label={item.title}
    >
        <div
            class="relative mb-3 aspect-square overflow
                       bg-[rgb(var(--mdui-color-surface-container-highest))]
                       {item.shape === 'circle'
                ? 'rounded-full'
                : 'rounded-t-xl'}"
        >
            {#if item.image}
                <img
                    src={item.image}
                    alt={item.title}
                    class="h-full w-full object-cover group-hover:scale-105"
                    loading="lazy"
                />
            {:else}
                <div
                    class="flex h-full w-full items-center justify-center text-4xl
                               text-[rgb(var(--mdui-color-on-surface-variant))]"
                >
                    {item.shape === 'circle' ? '👤' : '🎵'}
                </div>
            {/if}

            <div
                class="absolute inset-0 flex items-end justify-end p-3 opacity-0
                           transition-opacity duration-200 group-hover:opacity-100"
            >
                <mdui-button-icon variant="filled" icon="play_arrow--rounded"
                ></mdui-button-icon>
            </div>
        </div>

        <div class="min-w-0 px-2">
            <h2
                class="truncate text-sm font-semibold
                           text-[rgb(var(--mdui-color-on-surface))]"
            >
                {item.title}
            </h2>

            {#if item.subtitle}
                <p
                    class="mt-0 truncate text-xs
                               text-[rgb(var(--mdui-color-on-surface-variant))]"
                >
                    {item.subtitle}
                </p>
            {/if}
        </div>
    </mdui-card>
{/snippet}

{#if items.length === 0}
    <div
        class="flex min-h-60 flex-col items-center justify-center rounded-3xl
               border border-dashed border-[rgb(var(--mdui-color-outline-variant))]
               text-center"
    >
        <div class="text-5xl">🎧</div>
        <div class="mt-3 text-base font-medium">{emptyTitle}</div>
        <div
            class="mt-1 text-sm text-[rgb(var(--mdui-color-on-surface-variant))]"
        >
            {emptyDescription}
        </div>
    </div>
{:else}
    <section>
        <div
            class="grid grid-cols-2 gap-4
                   sm:grid-cols-3
                   md:grid-cols-4
                   lg:grid-cols-5
                   xl:grid-cols-6"
        >
            {#each items as item (item.id)}
                {@render mediaCard(item)}
            {/each}
        </div>
    </section>
{/if}
