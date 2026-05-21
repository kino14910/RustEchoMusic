<script>
    let {
        value = $bindable(0),
        cssStyle = '',
        width = 100,
        height = 12,
        thumbHeight = 18,
        onmousedown,
        onmouseover,
        onmousemove,
    } = $props()

    let startMove = $state(false)
    let dx = 0
    let inputEl

    let progressRaw = $derived(value / 100)
    let scale = $derived((width - 4) / width)

    function mouseDown(e) {
        startMove = true
        dx = e.offsetX
        let _value = clamp((e.offsetX / inputEl.offsetWidth) * 100, 0, 100)
        value = _value
        onmousedown?.(e)
    }

    function mouseMove(e) {
        e.stopPropagation()
        if (!startMove) return
        dx += e.movementX
        let _value = clamp((dx / inputEl.offsetWidth) * 100, 0, 100)
        value = _value
        onmousemove?.(e)
    }
    
    function handleKeydown(e) {
        if (e.key === 'ArrowUp' || e.key === 'ArrowRight') {
            e.preventDefault()
            value = clamp(value + 10, 0, 100)
            console.log(value)
        } else if (e.key === 'ArrowDown' || e.key === 'ArrowLeft') {
            e.preventDefault()
            value = clamp(value - 10, 0, 100)
        }
    }

    $effect(() => {
        if (startMove) {
            const onGlobalMouseUp = () => {
                startMove = false
                dx = 0
            }

            document.addEventListener('mousemove', mouseMove, {
                passive: true,
                capture: true,
            })
            document.addEventListener('mouseup', onGlobalMouseUp)

            return () => {
                document.removeEventListener('mousemove', mouseMove)
                document.removeEventListener('mouseup', onGlobalMouseUp)
            }
        }
    })

    function clamp(val, min, max) {
        return Math.min(max, Math.max(min, val))
    }
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
