<script lang="ts">
    type SliderProps = {
        value?: number
        cssStyle?: string
        width?: number
        height?: number
        thumbHeight?: number
        onmousedown?: (event: MouseEvent) => void
        onmouseover?: (event: MouseEvent | FocusEvent) => void
        onmousemove?: (event: MouseEvent) => void
    }

    let {
        value = $bindable(0),
        cssStyle = '',
        width = 100,
        height = 12,
        thumbHeight = 18,
        onmousedown,
        onmouseover,
        onmousemove,
    }: SliderProps = $props()

    let startMove = $state(false)
    let dx = 0
    let inputEl: HTMLDivElement | null = null

    let progressRaw = $derived(value / 100)
    let scale = $derived((width - 4) / width)

    function clamp(val: number, min: number, max: number): number {
        return Math.min(max, Math.max(min, val))
    }

    function getPercentFromOffset(offsetX: number): number {
        if (!inputEl) return value

        return clamp((offsetX / inputEl.offsetWidth) * 100, 0, 100)
    }

    function mouseDown(e: MouseEvent): void {
        if (!inputEl) return

        startMove = true
        dx = e.offsetX
        value = getPercentFromOffset(e.offsetX)

        onmousedown?.(e)
    }

    function mouseMove(e: MouseEvent): void {
        e.stopPropagation()

        if (!startMove || !inputEl) return

        dx += e.movementX
        value = clamp((dx / inputEl.offsetWidth) * 100, 0, 100)

        onmousemove?.(e)
    }

    function handleKeydown(e: KeyboardEvent): void {
        if (e.key === 'ArrowUp' || e.key === 'ArrowRight') {
            e.preventDefault()
            value = clamp(value + 10, 0, 100)
        } else if (e.key === 'ArrowDown' || e.key === 'ArrowLeft') {
            e.preventDefault()
            value = clamp(value - 10, 0, 100)
        }
    }

    $effect(() => {
        if (!startMove) return

        const onGlobalMouseUp = (): void => {
            startMove = false
            dx = 0
        }

        document.addEventListener('mousemove', mouseMove, {
            passive: true,
            capture: true
        })

        document.addEventListener('mouseup', onGlobalMouseUp)

        return () => {
            document.removeEventListener('mousemove', mouseMove, {
                capture: true
            })

            document.removeEventListener('mouseup', onGlobalMouseUp)
        }
    })
</script>

<div
    bind:this={inputEl}
    class="container"
    style="--width: {width}; --thumbHeight: {thumbHeight}; --scale: {scale}; {cssStyle}"
    onmousedown={mouseDown}
    {onmouseover}
    onfocus={onmouseover}
    onkeydown={handleKeydown}
    role="slider"
    aria-valuemin={0}
    aria-valuemax={100}
    aria-valuenow={value}
    aria-valuetext={`${Math.round(value)}%`}
    tabindex="0"
>
    <div class="track" style="--height: {height}; --progress: {progressRaw};">
        <div class="thumb-box"></div>
    </div>

    <div class="divider" style="--progress: {progressRaw};"></div>
</div>

<style>
    .container {
        --width: 100;
        width: calc(1px * var(--width));
        --thumbHeight: 18;
        position: relative;
        contain: paint;
        height: calc(var(--thumbHeight) * 1px);
    }

    .track {
        --progress: 0;
        --height: 12;
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        width: calc(100% - 2px);
        height: calc(var(--height) * 1px);
        border-radius: 12px;
        overflow: hidden;
    }

    .container > *,
    .track > * {
        pointer-events: none;
    }

    .thumb-box {
        width: 100%;
        height: calc(var(--height) * 1px);
        position: relative;
        display: flex;
        gap: 7px;
    }

    .thumb-box::before {
        content: '';
        position: relative;
        box-sizing: border-box;
        left: 0;
        width: calc(var(--progress) * var(--width) * 1px - 3px);
        height: 100%;
        border-radius: 12px 4px 4px 12px;
        background-color: rgb(var(--mdui-color-primary));
    }

    .thumb-box::after {
        content: '';
        box-sizing: border-box;
        position: relative;
        right: 0;
        width: calc((1 - var(--progress)) * var(--width) * 1px - 3px);
        height: 100%;
        border-radius: 4px 12px 12px 4px;
        background-color: rgb(
            var(--mdui-color-surface-container-highest-light)
        );
    }

    .divider {
        position: absolute;
        box-sizing: border-box;
        top: 50%;
        left: calc(var(--progress) * var(--scale) * 100% + 2px);
        transform: translate(-50%, -50%);
        display: flex;
        width: 1.5px;
        border: solid 1.5px rgb(var(--mdui-color-primary));
        border-radius: 3px;
        height: calc(var(--thumbHeight) * 1px);
        pointer-events: none;
        transition: all 0s ease;
    }
</style>