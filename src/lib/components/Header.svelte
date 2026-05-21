<script lang="ts">
    import { playerState, type SongInfo } from '$lib/player.svelte'
    import { invoke } from '@tauri-apps/api/core'
    import 'mdui/components/button.js'

    async function playMusic() {
        await invoke('play_music', {
            name: 'music/40mP 初音ミク - 恋愛裁判.flac',
        })

        playerState.isPlaying = true
        
        let song_info = await invoke<SongInfo>("get_song_info", {name: "music/40mP 初音ミク - 恋愛裁判.flac"})
        playerState.current = {
            title: song_info?.title,
            artist: song_info?.artist,
            path: "music/40mP 初音ミク - 恋愛裁判.flac"
        }
        playerState.duration = song_info.duration
    }
</script>

<header
    class="flex items-center justify-between px-8 py-4 bg-(--controlBackground) border-b border-(--controlGray) mt-14 shadow-sm"
>
    <div class="header-left">Header PlaceHolder</div>
    <mdui-button
        variant="filled"
        onclick={playMusic}
        onkeydown={(e: KeyboardEvent) => {
            if (e.key === 'Enter' || e.key === ' ') playMusic()
        }}
        class="px-4 py-2"
        role="none"
    >
        播放音乐
    </mdui-button>
</header>
