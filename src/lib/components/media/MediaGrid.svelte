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
        selectedId = null,
        onselect,
        onplay,
    }: {
        items: MediaGridItem[]
        emptyTitle?: string
        emptyDescription?: string
        selectedId?: string | number | null
        onselect?: (item: MediaGridItem) => void
        onplay?: (item: MediaGridItem, event: Event) => void
    } = $props()

    function handleSelect(item: MediaGridItem) {
        onselect?.(item)
    }

    function handleKeydown(item: MediaGridItem, event: KeyboardEvent) {
        if (event.key !== 'Enter' && event.key !== ' ') return

        event.preventDefault()
        handleSelect(item)
    }

    function handlePlay(item: MediaGridItem, event: Event) {
        event.preventDefault()
        event.stopPropagation()
        onplay?.(item, event)
    }

    function handlePlayKeydown(item: MediaGridItem, event: KeyboardEvent) {
        if (event.key !== 'Enter' && event.key !== ' ') return

        handlePlay(item, event)
    }
</script>

{#snippet mediaCardContent(item: MediaGridItem, isSelected: boolean)}
    <div
        class="relative mb-3 aspect-square
               bg-[rgb(var(--mdui-color-surface-container-highest))]
               {item.shape === 'circle' ? 'rounded-full' : 'rounded-t-xl'}"
    >
        {#if item.image}
            <img
                src={item.image}
                alt={item.title}
                class="h-full w-full object-cover transition-transform duration-200 group-hover:scale-105"
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

        {#if onplay}
            <div
                class="absolute inset-0 flex items-end justify-end p-2 opacity-0
                       transition-opacity duration-200 group-hover:opacity-100 group-focus-within:opacity-100"
            >
                <mdui-button-icon
                    variant="filled"
                    icon="play_arrow--rounded"
                    aria-label={`播放 ${item.title}`}
                    role="button"
                    tabindex="0"
                    onclick={(event: Event) => handlePlay(item, event)}
                    onkeydown={(event: KeyboardEvent) =>
                        handlePlayKeydown(item, event)}
                ></mdui-button-icon>
            </div>
        {/if}
    </div>

    <div class="min-w-0 px-2 pb-2">
        <h2
            class={[
                'truncate text-sm font-semibold',
                isSelected
                    ? 'text-[rgb(var(--mdui-color-on-secondary-container))]'
                    : 'text-[rgb(var(--mdui-color-on-surface))]',
            ]}
        >
            {item.title}
        </h2>

        {#if item.subtitle}
            <p
                class={[
                    'mt-0 truncate text-xs',
                    isSelected
                        ? 'text-[rgb(var(--mdui-color-on-secondary-container))]'
                        : 'text-[rgb(var(--mdui-color-on-surface-variant))]',
                ]}
            >
                {item.subtitle}
            </p>
        {/if}
    </div>
{/snippet}

{#snippet mediaCard(item: MediaGridItem)}
    {@const isSelected = selectedId === item.id}
    {#if onselect}
        <mdui-card
            class={[
                'group cursor-pointer outline-none transition-colors hover:bg-(--mdui-color-secondary-container)/80',
                isSelected && 'bg-[rgb(var(--mdui-color-secondary-container))]',
            ]}
            aria-label={item.title}
            aria-pressed={isSelected}
            onclick={() => handleSelect(item)}
            onkeydown={(event: KeyboardEvent) => handleKeydown(item, event)}
            role="button"
            tabindex="0"
        >
            {@render mediaCardContent(item, isSelected)}
        </mdui-card>
    {:else}
        <mdui-card
            href={item.href ?? undefined}
            class={[
                'group outline-none transition-colors bg-(--mdui-color-secondary-container)/80',
                isSelected && 'bg-[rgb(var(--mdui-color-secondary-container))]',
            ]}
            aria-label={item.title}
        >
            {@render mediaCardContent(item, isSelected)}
        </mdui-card>
    {/if}
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
