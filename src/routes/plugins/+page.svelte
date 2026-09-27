<script lang="ts">
    import Heading from '$lib/components/base/Heading.svelte'
    import { pluginState } from '$lib/state/plugins.svelte'
    import 'mdui/components/circular-progress.js'
    import 'mdui/components/switch.js'
    import { onMount } from 'svelte'
    import { goto } from '$app/navigation'
    import { base } from '$app/paths'
    import type { KernelPlugin } from '$lib/types'

    const STATE_LABELS: Record<string, string> = {
        active: '已启用',
        activating: '启用中',
        deactivating: '停用中',
        failed: '启用失败',
        stopped: '已停止',
        unloaded: '已卸载',
        discovered: '已发现',
        resolved: '已解析',
        loaded: '已加载',
    }

    let busyPlugin = $state<string | null>(null)
    // 开关的乐观覆盖：点下去先按目标状态显示，命令结束后清掉，
    // 回落到内核快照的真实状态——加载失败时开关会自动弹回去。
    let pending = $state<Record<string, boolean>>({})

    onMount(() => {
        void pluginState.refreshExtensions()
    })

    function handlePluginClick(route: string) {
        if (route) void goto(`${base}${route}`)
    }

    function kernelEntry(id: string): KernelPlugin | undefined {
        return pluginState.kernel?.plugins.find(plugin => plugin.id === id)
    }

    function stateLabel(state: string | undefined): string {
        if (!state) return '未知'
        return STATE_LABELS[state] ?? state
    }

    function isEnabled(id: string): boolean {
        const override = pending[id]
        if (override !== undefined) return override
        return kernelEntry(id)?.active ?? true
    }

    async function togglePlugin(id: string, next: boolean) {
        const entry = kernelEntry(id)
        if (!entry || !entry.userDisableable || busyPlugin) return

        busyPlugin = id
        pending = { ...pending, [id]: next }
        try {
            if (next) {
                await pluginState.enablePlugin(id)
            } else {
                await pluginState.disablePlugin(id)
            }
        } finally {
            const rest = { ...pending }
            delete rest[id]
            pending = rest
            busyPlugin = null
        }
    }
</script>

<svelte:head>
    <title>插件</title>
</svelte:head>

<section class="flex h-full min-h-0 flex-col gap-6 overflow-auto pb-10">
    <Heading eyebrow="Plugins" title="插件" />

    <a
        href="{base}/plugins/kernel"
        class="self-start rounded-full bg-[rgb(var(--mdui-color-secondary-container))] px-3 py-1 text-xs text-[rgb(var(--mdui-color-on-secondary-container))] transition-colors hover:bg-[rgb(var(--mdui-color-secondary-container-high))]"
    >
        查看插件内核 / 贡献点 →
    </a>

    <div
        class="flex flex-1 items-center justify-center"
        class:hidden={!pluginState.isLoading}
    >
        <mdui-circular-progress></mdui-circular-progress>
    </div>

    <div class="grid gap-4" class:hidden={pluginState.isLoading}>
        {#if pluginState.manifests.length === 0}
            <div
                class="rounded-2xl border border-dashed border-[rgb(var(--mdui-color-outline-variant))] p-8 text-center text-sm text-[rgb(var(--mdui-color-on-surface-variant))]"
            >
                暂无已注册的插件。
            </div>
        {:else}
            {#each pluginState.manifests as manifest (manifest.id)}
                {@const entry = kernelEntry(manifest.id)}
                {@const state = stateLabel(entry?.state)}
                <div
                    class="flex items-center gap-4 rounded-2xl bg-[rgb(var(--mdui-color-surface-container))] p-5 transition-colors hover:bg-[rgb(var(--mdui-color-surface-container-high))]"
                >
                    <button
                        type="button"
                        class="flex flex-1 items-center gap-4 border-0 bg-transparent p-0 text-left"
                        onclick={() => handlePluginClick(manifest.route)}
                        onkeydown={(e: KeyboardEvent) => {
                            if (e.key === 'Enter' || e.key === ' ') {
                                handlePluginClick(manifest.route)
                            }
                        }}
                    >
                        <div
                            class="flex h-12 w-12 items-center justify-center rounded-xl bg-[rgb(var(--mdui-color-primary-container))] text-[rgb(var(--mdui-color-on-primary-container))]"
                        >
                            <mdui-icon name="extension--rounded"></mdui-icon>
                        </div>
                        <div class="flex flex-1 flex-col gap-1">
                            <div class="text-sm font-medium">
                                {manifest.displayName}
                            </div>
                            <div
                                class="text-xs text-[rgb(var(--mdui-color-on-surface-variant))]"
                            >
                                {manifest.id}
                            </div>
                        </div>
                    </button>

                    <div
                        class="rounded-full px-2 py-0.5 text-xs font-medium"
                        title={entry?.lastError ?? undefined}
                        class:bg-green-100={entry?.state === 'active'}
                        class:text-green-700={entry?.state === 'active'}
                        class:bg-red-100={entry?.state === 'failed'}
                        class:text-red-700={entry?.state === 'failed'}
                        class:bg-gray-100={entry?.state !== 'active' &&
                            entry?.state !== 'failed'}
                        class:text-gray-500={entry?.state !== 'active' &&
                            entry?.state !== 'failed'}
                    >
                        {state}
                    </div>

                    {#if entry && !entry.userDisableable}
                        <span
                            class="rounded-full bg-[rgb(var(--mdui-color-surface-container-highest))] px-2 py-0.5 text-[10px] text-[rgb(var(--mdui-color-on-surface-variant))]"
                            title="核心插件不可禁用"
                        >
                            核心
                        </span>
                    {:else}
                        <mdui-switch
                            checked={isEnabled(manifest.id)}
                            disabled={busyPlugin === manifest.id}
                            aria-label={isEnabled(manifest.id)
                                ? `禁用 ${manifest.displayName}`
                                : `启用 ${manifest.displayName}`}
                            onchange={(event: Event) => {
                                const target = event.currentTarget as HTMLElement & {
                                    checked: boolean
                                }
                                void togglePlugin(manifest.id, target.checked)
                            }}
                        ></mdui-switch>
                    {/if}
                </div>
            {/each}
        {/if}

        {#if pluginState.error}
            <div class="text-sm text-red-500">
                {pluginState.error}
            </div>
        {/if}
    </div>
</section>
