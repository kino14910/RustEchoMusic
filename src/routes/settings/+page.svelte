<script lang="ts">
    import Button from '$lib/components/base/Button.svelte'
    import Heading from '$lib/components/base/Heading.svelte'
    import Select from '$lib/components/base/Select.svelte'
    import TextField from '$lib/components/base/TextField.svelte'
    import { registerBuiltinExtensions } from '$lib/extensions/builtin'
    import { extensionRegistry } from '$lib/extensions/registry.svelte'
    import { settings } from '$lib/state/settings.svelte'
    import type { ThemeMode } from '$lib/types'
    import { onMount } from 'svelte'

    import 'mdui/components/circular-progress.js'
    import 'mdui/components/switch.js'

    let newLibraryDir = $state('')

    const themeOptions: { label: string; value: ThemeMode }[] = [
        { label: '跟随系统', value: 'system' },
        { label: '浅色模式', value: 'light' },
        { label: '深色模式', value: 'dark' },
    ]

    onMount(() => {
        registerBuiltinExtensions()
        void settings.load()
    })

    function addLibraryDir() {
        settings.addLibraryDir(newLibraryDir)
        newLibraryDir = ''
    }

    function handleSwitchChange(
        event: Event,
        key: 'scanOnStartup' | 'reduceMotion',
    ) {
        const target = event.currentTarget as HTMLElement & {
            checked: boolean
        }

        settings.update({
            [key]: target.checked,
        })
    }
</script>

<svelte:head>
    <title>设置</title>
</svelte:head>

<section class="flex h-full min-h-0 flex-col gap-6 overflow-auto pb-10">
    <Heading eyebrow="Settings" title="设置" />

    {#if settings.isLoading}
        <div class="flex flex-1 items-center justify-center">
            <mdui-circular-progress></mdui-circular-progress>
        </div>
    {:else}
        <div class="grid gap-6">
            <section
                class="rounded-3xl bg-[rgb(var(--mdui-color-surface-container))] p-5"
            >
                <h2
                    class="text-lg font-semibold text-[rgb(var(--mdui-color-on-surface))]"
                >
                    外观
                </h2>

                <div class="mt-4 grid gap-4">
                    <div class="flex items-center justify-between gap-4">
                        <div>
                            <div class="text-sm font-medium">主题</div>
                            <div
                                class="text-xs text-[rgb(var(--mdui-color-on-surface-variant))]"
                            >
                                设置应用的明暗模式
                            </div>
                        </div>

                        <div class="w-48">
                            <Select
                                value={settings.data.theme}
                                options={themeOptions}
                                onchange={(value) => {
                                    settings.update({
                                        theme: value as ThemeMode,
                                    })
                                }}
                            />
                        </div>
                    </div>

                    <div class="flex items-center justify-between gap-4">
                        <div>
                            <div class="text-sm font-medium">减少动画</div>
                            <div
                                class="text-xs text-[rgb(var(--mdui-color-on-surface-variant))]"
                            >
                                降低界面动画和过渡效果
                            </div>
                        </div>

                        <mdui-switch
                            checked={settings.data.reduceMotion}
                            onchange={(event: Event) =>
                                handleSwitchChange(event, 'reduceMotion')}
                        ></mdui-switch>
                    </div>
                </div>
            </section>

            <section
                class="rounded-3xl bg-[rgb(var(--mdui-color-surface-container))] p-5"
            >
                <h2
                    class="text-lg font-semibold text-[rgb(var(--mdui-color-on-surface))]"
                >
                    媒体库
                </h2>

                <div class="mt-4 grid gap-4">
                    <div class="flex items-center justify-between gap-4">
                        <div>
                            <div class="text-sm font-medium">启动时扫描</div>
                            <div
                                class="text-xs text-[rgb(var(--mdui-color-on-surface-variant))]"
                            >
                                应用启动时自动刷新媒体库
                            </div>
                        </div>

                        <mdui-switch
                            checked={settings.data.scanOnStartup}
                            onchange={(event: Event) =>
                                handleSwitchChange(event, 'scanOnStartup')}
                        ></mdui-switch>
                    </div>

                    <div class="grid gap-3">
                        <div>
                            <div class="text-sm font-medium">媒体库目录</div>
                            <div
                                class="text-xs text-[rgb(var(--mdui-color-on-surface-variant))]"
                            >
                                后续可以接入 Tauri 文件夹选择器
                            </div>
                        </div>

                        <div class="flex gap-2">
                            <TextField
                                bind:value={newLibraryDir}
                                variant="outlined"
                                placeholder="输入音乐目录路径"
                            />

                            <Button variant="filled" onclick={addLibraryDir}>
                                添加
                            </Button>
                        </div>

                        {#if settings.data.libraryDirs.length > 0}
                            <div class="grid gap-2">
                                {#each settings.data.libraryDirs as dir (dir)}
                                    <div
                                        class="flex items-center justify-between gap-3 rounded-2xl bg-[rgb(var(--mdui-color-surface-container-highest))] px-4 py-3"
                                    >
                                        <div class="truncate text-sm">
                                            {dir}
                                        </div>

                                        <Button
                                            variant="text"
                                            onclick={() =>
                                                settings.removeLibraryDir(dir)}
                                        >
                                            移除
                                        </Button>
                                    </div>
                                {/each}
                            </div>
                        {:else}
                            <div
                                class="rounded-2xl border border-dashed border-[rgb(var(--mdui-color-outline-variant))] p-4 text-sm text-[rgb(var(--mdui-color-on-surface-variant))]"
                            >
                                暂未添加媒体库目录
                            </div>
                        {/if}
                    </div>
                </div>
            </section>

            <section
                class="rounded-3xl bg-[rgb(var(--mdui-color-surface-container))] p-5"
            >
                <h2
                    class="text-lg font-semibold text-[rgb(var(--mdui-color-on-surface))]"
                >
                    扩展
                </h2>

                <div class="mt-4 grid gap-3">
                    {#each extensionRegistry.extensions as extension (extension.id)}
                        <div
                            class="flex items-center justify-between gap-4 rounded-2xl bg-[rgb(var(--mdui-color-surface-container-highest))] px-4 py-3"
                        >
                            <div class="min-w-0">
                                <div class="text-sm font-medium">
                                    {extension.name}
                                </div>

                                {#if extension.description}
                                    <div
                                        class="mt-1 text-xs text-[rgb(var(--mdui-color-on-surface-variant))]"
                                    >
                                        {extension.description}
                                    </div>
                                {/if}
                            </div>

                            <mdui-switch
                                checked={extension.enabled !== false}
                                onchange={(event: Event) => {
                                    const target =
                                        event.currentTarget as HTMLElement & {
                                            checked: boolean
                                        }

                                    extensionRegistry.setEnabled(
                                        extension.id,
                                        target.checked,
                                    )
                                }}
                            ></mdui-switch>
                        </div>
                    {/each}
                </div>
            </section>

            {#if settings.error}
                <div class="text-sm text-red-500">
                    {settings.error}
                </div>
            {/if}
        </div>
    {/if}
</section>