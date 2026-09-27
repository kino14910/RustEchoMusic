<script lang="ts">
    import { onMount, tick } from 'svelte'
    import { invoke } from '@tauri-apps/api/core'
    import { getCurrentWindow } from '@tauri-apps/api/window'
    import { LogicalSize } from '@tauri-apps/api/dpi'
    import IconButton from '$lib/components/base/IconButton.svelte'

    interface SnapshotLine {
        timestampMs: number
        text: string
    }

    interface LyricsSnapshot {
        playing: boolean
        positionMs: number
        activeIndex: number | null
        lines: SnapshotLine[]
    }

    const POLL_INTERVAL_MS = 250
    const CONTEXT_LINES = 1
    const MIN_WINDOW_HEIGHT = 72
    const WINDOW_VERTICAL_PADDING = 36
    const MAX_CONSECUTIVE_FAILURES = 60

    const appWindow = getCurrentWindow()

    let lyricsBox = $state<HTMLDivElement | null>(null)
    let appliedHeight = 0

    let snapshot = $state<LyricsSnapshot | null>(null)
    let errorText = $state<string | null>(null)
    let consecutiveFailures = 0
    let timer: number | null = null

    const activeIndex = $derived(snapshot?.activeIndex ?? null)

    const windowedLines = $derived.by(() => {
        const lines = snapshot?.lines ?? []
        if (lines.length === 0) return []
        const focus = activeIndex ?? 0
        const start = Math.max(0, focus - CONTEXT_LINES)
        const end = Math.min(lines.length, focus + CONTEXT_LINES + 1)
        return lines
            .slice(start, end)
            .map((line, offset) => ({
                text: typeof line?.text === 'string' ? line.text : '',
                index: start + offset,
            }))
            .filter(line => line.text.length > 0)
    })

    function lineClass(index: number): string {
        return index === activeIndex
            ? 'text-2xl font-semibold text-white'
            : 'text-base text-white/60'
    }

    // 窗口高度跟着实际渲染出来的歌词行数走：CONTEXT_LINES 一改，
    // 或者行数从"暂无歌词"变到多行，窗口自己收放，不必回插件改常量。
    async function fitWindowToLyrics() {
        const box = lyricsBox
        if (!box) return

        const target = Math.max(
            MIN_WINDOW_HEIGHT,
            Math.ceil(box.getBoundingClientRect().height + WINDOW_VERTICAL_PADDING),
        )
        if (Math.abs(target - appliedHeight) < 2) return
        appliedHeight = target

        try {
            const scale = await appWindow.scaleFactor()
            const size = await appWindow.innerSize()
            await appWindow.setSize(new LogicalSize(size.width / scale, target))
        } catch (error) {
            console.warn('[desktop-lyrics] 窗口高度自适应失败:', error)
        }
    }

    $effect(() => {
        if (!lyricsBox) return
        void windowedLines.length
        void tick().then(fitWindowToLyrics)
    })

    async function readSnapshot() {
        try {
            const next = await invoke<LyricsSnapshot | null>(
                'execute_plugin_command',
                { commandId: 'desktop-lyrics.snapshot', args: 'None' },
            )
            snapshot = next && Array.isArray(next.lines) ? next : null
            errorText = null
            consecutiveFailures = 0
        } catch (error) {
            snapshot = null
            errorText = describeError(error)
            consecutiveFailures += 1
            if (consecutiveFailures === 1) {
                console.warn('[desktop-lyrics] 读取歌词快照失败:', error)
            }
            // 插件被禁用/卸载后命令会一直失败，窗口没有存在意义，自己退场。
            if (consecutiveFailures >= MAX_CONSECUTIVE_FAILURES) {
                void appWindow.close()
            }
        }
    }

    function scheduleRead() {
        const backoff = Math.min(5_000, POLL_INTERVAL_MS * 2 ** consecutiveFailures)
        timer = window.setTimeout(async () => {
            await readSnapshot()
            scheduleRead()
        }, backoff)
    }

    function startDrag(event: MouseEvent) {
        if (event.button !== 0) return
        void appWindow.startDragging()
    }

    function blockDrag(event: MouseEvent) {
        event.stopPropagation()
    }

    let controlling = $state(false)

    function describeError(error: unknown): string {
        if (typeof error === 'string') return error
        if (error instanceof Error) return error.message
        if (error && typeof error === 'object') {
            for (const value of Object.values(error as Record<string, unknown>)) {
                if (typeof value === 'string') return value
            }
            try {
                return JSON.stringify(error)
            } catch {
                return String(error)
            }
        }
        return String(error)
    }

    async function sendControl(command: string) {
        if (controlling) return
        controlling = true
        try {
            await invoke('execute_plugin_command', {
                commandId: command,
                args: 'None',
            })
            errorText = null
            await readSnapshot()
        } catch (error) {
            errorText = describeError(error)
            console.warn('[desktop-lyrics] 播放控制失败:', error)
        } finally {
            controlling = false
        }
    }

    onMount(() => {
        void readSnapshot().then(scheduleRead)
        return () => {
            if (timer !== null) window.clearTimeout(timer)
        }
    })
</script>

<svelte:head>
    <style>
        html,
        body {
            background: transparent !important;
            overflow: hidden !important;
            margin: 0 !important;
            padding: 0 !important;
        }
    </style>
</svelte:head>

<div
    role="presentation"
    onmousedown={startDrag}
    class="group relative flex h-screen w-screen select-none items-center justify-center overflow-hidden px-6 text-center transition-colors duration-200 hover:bg-[#00000033]"
>
    <div
        bind:this={lyricsBox}
        class="flex flex-col items-center gap-1"
    >
        {#if windowedLines.length === 0}
            <p class="lyric-line text-sm text-white/70">暂无歌词</p>
        {:else}
            {#each windowedLines as line (line.index)}
                <p
                    class="lyric-line max-w-full truncate transition-all duration-200 {lineClass(
                        line.index,
                    )}"
                >
                    {line.text}
                </p>
            {/each}
        {/if}
    </div>

    {#if errorText}
        <p
            class="absolute top-1 left-1/2 max-w-[92vw] -translate-x-1/2 truncate text-[10px] text-red-300"
            title={errorText}
        >
            {errorText}
        </p>
    {/if}

    <div
        class="absolute bottom-3 left-1/2 flex -translate-x-1/2 flex-col items-center gap-1 opacity-0 transition-opacity duration-200 group-hover:opacity-100"
    >
        <div class="transport-controls flex items-center gap-2">
            <IconButton
                icon="skip_previous--rounded"
                title="上一首"
                disabled={controlling}
                onmousedown={blockDrag}
                onclick={() => void sendControl('desktop-lyrics.previous')}
            />
            <IconButton
                variant="filled"
                icon={snapshot?.playing ? 'pause--rounded' : 'play_arrow--rounded'}
                title={snapshot?.playing ? '暂停' : '播放'}
                disabled={controlling}
                onmousedown={blockDrag}
                onclick={() => void sendControl('desktop-lyrics.playPause')}
            />
            <IconButton
                icon="skip_next--rounded"
                title="下一首"
                disabled={controlling}
                onmousedown={blockDrag}
                onclick={() => void sendControl('desktop-lyrics.next')}
            />
        </div>
    </div>
</div>

<style>
    .lyric-line {
        color: var(--mdui-color-on-primary-container);
    }

    /* 窗口是透明暗色背景，把 mdui 的语义色就地改成白色系，
       让 IconButton 与 PlayerBar 的传输控件视觉一致且任何桌面壁纸上都看得清。 */
    /* .transport-controls {
        color: #fff;
        --mdui-color-on-surface: 255 255 255;
        --mdui-color-on-surface-variant: 255 255 255;
        --mdui-color-primary: 255 255 255;
        --mdui-color-on-primary: 32 32 32;
    } */
</style>
