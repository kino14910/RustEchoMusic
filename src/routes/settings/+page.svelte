<script lang="ts">
    import Button from '$lib/components/base/Button.svelte'
    import Heading from '$lib/components/base/Heading.svelte'
    import Select from '$lib/components/base/Select.svelte'
    import TextField from '$lib/components/base/TextField.svelte'
    import { registerBuiltinExtensions } from '$lib/extensions/builtin'
    import { extensionRegistry } from '$lib/extensions/registry.svelte'
    import SettingsListItem from '$lib/features/settings/SettingsListItem.svelte'
    import SettingsRow from '$lib/features/settings/SettingsRow.svelte'
    import SettingsSection from '$lib/features/settings/SettingsSection.svelte'
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
            <SettingsSection title="外观">
                <SettingsRow title="主题" description="设置应用的明暗模式">
                    <div class="w-48">
                        <Select
                            value={settings.data.theme}
                            options={themeOptions}
                            onchange={value => {
                                settings.update({
                                    theme: value as ThemeMode,
                                })
                            }}
                        />
                    </div>
                </SettingsRow>

                <SettingsRow
                    title="减少动画"
                    description="降低界面动画和过渡效果"
                >
                    <mdui-switch
                        checked={settings.data.reduceMotion}
                        onchange={(event: Event) =>
                            handleSwitchChange(event, 'reduceMotion')}
                    ></mdui-switch>
                </SettingsRow>
            </SettingsSection>

            <SettingsSection title="媒体库">
                <SettingsRow
                    title="启动时扫描"
                    description="应用启动时自动刷新媒体库"
                >
                    <mdui-switch
                        checked={settings.data.scanOnStartup}
                        onchange={(event: Event) =>
                            handleSwitchChange(event, 'scanOnStartup')}
                    ></mdui-switch>
                </SettingsRow>

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
                                <SettingsListItem title={dir}>
                                    <Button
                                        variant="text"
                                        onclick={() =>
                                            settings.removeLibraryDir(dir)}
                                    >
                                        移除
                                    </Button>
                                </SettingsListItem>
                            {/each}
                        </div>
                    {:else}
                        <div
                            class={`rounded-2xl border border-dashed border-[rgb(var(--mdui-color-outline-variant))] p-4 text-sm text-[rgb(var(--mdui-color-on-surface-variant))]`}
                        >
                            暂未添加媒体库目录
                        </div>
                    {/if}
                </div>
            </SettingsSection>

            <SettingsSection title="扩展" contentClass="mt-4 grid gap-3">
                {#each extensionRegistry.extensions as extension (extension.id)}
                    <SettingsListItem
                        title={extension.name}
                        description={extension.description}
                    >
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
                    </SettingsListItem>
                {/each}
            </SettingsSection>

            {#if settings.error}
                <div class="text-sm text-red-500">
                    {settings.error}
                </div>
            {/if}
        </div>
    {/if}
</section>
