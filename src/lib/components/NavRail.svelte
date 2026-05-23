<script lang="ts">
  import { player, type TrackInfo } from '$lib/player.svelte'
  import { invoke } from '@tauri-apps/api/core'
  import { open } from '@tauri-apps/plugin-dialog'
  import 'mdui/components/navigation-rail-item.js'
  import 'mdui/components/navigation-rail.js'

  async function importMusicFolder(): Promise<TrackInfo[] | undefined> {
    const selectedDir = await open({
      directory: true,
      multiple: false,
      title: '选择你的音乐库文件夹',
    })

    if (!selectedDir) {
      console.error('音频文件不存在或已被移动')
      return
    }

    try {
      const tracks: TrackInfo[] = await invoke('scan_music_directory', {
        dirPath: selectedDir,
      })

      console.log(`成功扫描到 ${tracks.length} 首歌曲！`, tracks)
      return tracks
    } catch (err) {
      console.error('扫描音乐文件夹失败:', err)
    }
  }

  async function playMusic() {
    const playlist = await importMusicFolder()
    if (!playlist) return

    player.playlist = playlist
    player.playByIndex(0)
  }
</script>

<mdui-navigation-rail style="--z-index: 1">
  <mdui-fab
    lowered
    icon="playlist_add--rounded"
    slot="top"
    onclick={playMusic}
    onkeydown={(e: KeyboardEvent) => {
      if (e.key === 'Enter' || e.key === ' ') playMusic()
    }}
    role="button"
    tabindex="0"
  ></mdui-fab>
  <!-- <mdui-button-icon icon="settings" slot="bottom"></mdui-button-icon> -->

  <mdui-navigation-rail-item icon="watch_later--outlined" class="mb-4"
    >Recent</mdui-navigation-rail-item
  >
  <mdui-navigation-rail-item icon="library_music--outlined"
    >Library</mdui-navigation-rail-item
  >
  <mdui-navigation-rail-item icon="track_changes--outlined"
    >Track</mdui-navigation-rail-item
  >
  <mdui-navigation-rail-item icon="person--outlined"
    >Artist</mdui-navigation-rail-item
  >
</mdui-navigation-rail>
