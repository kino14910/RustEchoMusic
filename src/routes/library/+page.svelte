<script lang="ts">
    import TrackList from '$lib/features/track-list/TrackList.svelte'
    import { musicLibrary } from '$lib/state/library.svelte'
    import { player } from '$lib/state/player.svelte'
    import type { Track } from '$lib/types'

    import Button from '$lib/components/base/Button.svelte'
    import Heading from '$lib/components/base/Heading.svelte'
    import IconButton from '$lib/components/base/IconButton.svelte'
    import SearchBar from '$lib/components/base/SearchBar.svelte'
    import SettingsListItem from '$lib/features/settings/SettingsListItem.svelte'
    import SettingsRow from '$lib/features/settings/SettingsRow.svelte'
    import { settings } from '$lib/state/settings.svelte'
    import {
      removeMusicDirectory,
      scanDirectory,
    } from '$lib/utils/library'
    import { invoke } from '@tauri-apps/api/core'

    import '@mdui/icons/arrow-back--rounded.js'
    import '@mdui/icons/checklist-rtl--outlined.js'
    import '@mdui/icons/delete--rounded.js'
    import '@mdui/icons/delete-forever--rounded.js'
    import '@mdui/icons/delete-outline--rounded.js'
    import '@mdui/icons/folder--outlined.js'
    import '@mdui/icons/play-arrow--rounded.js'
    import '@mdui/icons/playlist-add--outlined.js'
    import '@mdui/icons/playlist-play--rounded.js'
    import '@mdui/icons/refresh--rounded.js'
    import 'mdui/components/button.js'
    import 'mdui/components/circular-progress.js'
    import 'mdui/components/dialog.js'
    import 'mdui/components/segmented-button-group.js'
    import 'mdui/components/segmented-button.js'
    import 'mdui/components/snackbar.js'
    import 'mdui/components/switch.js'

    type SortBy = 'title' | 'artist' | 'album'

    let searchQuery = $state('')
    let sortBy = $state<SortBy>('title')
    let isBatchMode = $state(false)
    let selectedTrackIds = $state<number[]>([])
    let isLibraryDialogOpen = $state(false)
    let isAddAllSnackbarOpen = $state(false)
    let isDeleteConfirmDialogOpen = $state(false)

    const collator = new Intl.Collator('zh-Hans-CN', {
        numeric: true,
        sensitivity: 'base',
    })

    const searchResults = $derived(
        musicLibrary.tracks.filter(
            track =>
                (track.title ?? '')
                    .toLowerCase()
                    .includes(searchQuery.toLowerCase()) ||
                (track.artist ?? '')
                    .toLowerCase()
                    .includes(searchQuery.toLowerCase()) ||
                (track.album ?? '')
                    .toLowerCase()
                    .includes(searchQuery.toLowerCase()),
        ),
    )

    let sortedCache: Track[] = []
    let lastSortBy: SortBy = 'title'
    let lastTrackLength = 0

    const defaultSortedTracks = $derived.by(() => {
        const tracks = musicLibrary.tracks
        if (sortBy === lastSortBy && tracks.length === lastTrackLength && sortedCache.length > 0) {
            return sortedCache
        }
        sortedCache = tracks.toSorted((a, b) =>
            collator.compare(a[sortBy] ?? '', b[sortBy] ?? ''),
        )
        lastSortBy = sortBy
        lastTrackLength = tracks.length
        return sortedCache
    })

    const displayTracks = $derived(
        searchQuery ? searchResults : defaultSortedTracks,
    )

    const selectedTracks = $derived(
        displayTracks.filter(track => selectedTrackIds.includes(track.id)),
    )

    function appendTracksToPlaylist(tracks: Track[]) {
        player.insertTracksAsNext(tracks)
    }

    function addAllToPlaylist() {
        appendTracksToPlaylist(displayTracks)
        isAddAllSnackbarOpen = true
    }

    function closeBatchMode() {
        selectedTrackIds = []
        isBatchMode = false
        isDeleteConfirmDialogOpen = false
    }

    function toggleBatchMode() {
        if (isBatchMode) {
            closeBatchMode()
            return
        }

        isBatchMode = true
    }

    function toggleTrackSelection(track: Track, checked: boolean) {
        if (checked) {
            if (!selectedTrackIds.includes(track.id)) {
                selectedTrackIds = [...selectedTrackIds, track.id]
            }

            return
        }

        selectedTrackIds = selectedTrackIds.filter(id => id !== track.id)
    }

    function addSelectedToPlaylist() {
        appendTracksToPlaylist(selectedTracks)
    }

    function getSelectedTrackIds() {
        return selectedTracks.map(track => track.id)
    }

    function removeSelectedFromCurrentList() {
        const selectedIds = new Set(selectedTrackIds)
        musicLibrary.tracks = musicLibrary.tracks.filter(
            track => !selectedIds.has(track.id),
        )
        closeBatchMode()
    }

    async function moveSelectedToTrash() {
        const trackIds = getSelectedTrackIds()

        try {
            await invoke('trash_track_files', { trackIds })
            await musicLibrary.scan()
            closeBatchMode()
        } catch (err) {
            console.error('移至回收站失败:', err)
            musicLibrary.error = String(err)
        }
    }

    async function deleteSelectedFromDisk() {
        const trackIds = getSelectedTrackIds()

        try {
            await invoke('delete_track_files', { trackIds })
            await musicLibrary.scan()
            closeBatchMode()
        } catch (err) {
            console.error('删除本地音乐文件失败:', err)
            musicLibrary.error = String(err)
        }
    }

    function handleSwitchChange(event: Event, key: 'scanOnStartup') {
        const target = event.currentTarget as HTMLElement & {
            checked: boolean
        }

        settings.update({
            [key]: target.checked,
        })
    }

    function playAll() {
        if (displayTracks.length === 0) return
        player.replacePlaylistAndPlay(displayTracks, displayTracks[0].id)
    }

    function playContextTrack(track: Track) {
        player.replacePlaylistAndPlay(displayTracks, track.id)
    }

    async function insertContextTrackNext(track: Track) {
        await player.insertTrackAsNext(track)
    }

    async function showContextTrackFolder(track: Track) {
        await invoke('show_in_folder', { trackId: track.id })
    }

    async function removeContextTrackFromList(track: Track) {
        musicLibrary.tracks = musicLibrary.tracks.filter(item => item.id !== track.id)
        await player.removeTrack(track.id)
    }

    async function deleteContextTrackFile(track: Track) {
        await invoke('delete_track_file', { trackId: track.id })
        await musicLibrary.scan()
    }

    function handleSortChange(event: Event) {
        const value = (event.currentTarget as HTMLElement & { value: string })
            .value
        if (value === 'title' || value === 'artist' || value === 'album') {
            sortBy = value
        }
    }
</script>

<svelte:head>
    <title>歌曲</title>
</svelte:head>

<div class="flex flex-col h-full overflow-hidden">
    <header
        class="border-b border-[rgb(var(--mdui-color-outline-variant))] pb-4 shrink-0"
    >
        <Heading eyebrow="library" title="歌曲" />
        <div class="flex justify-between">
            <div class="flex items-center gap-2">
                <Button
                    variant="filled"
                    icon="play_arrow--rounded"
                    disabled={displayTracks.length === 0}
                    onclick={playAll}
                >
                    播放全部
                </Button>

                <IconButton
                    icon="refresh--rounded"
                    variant="tonal"
                    onclick={() => musicLibrary.scan()}
                />

                <mdui-tooltip content="添加全部至播放列表">
                    <IconButton
                        variant="tonal"
                        icon="playlist_add--outlined"
                        disabled={displayTracks.length === 0}
                        onclick={addAllToPlaylist}
                    />
                </mdui-tooltip>

                <mdui-tooltip content="批量操作">
                    <IconButton
                        variant={isBatchMode ? 'filled' : 'tonal'}
                        icon="checklist_rtl--outlined"
                        onclick={toggleBatchMode}
                    />
                </mdui-tooltip>

                <mdui-tooltip content="管理媒体文件夹">
                    <IconButton
                        variant="tonal"
                        icon="folder--outlined"
                        onclick={() => {
                            isLibraryDialogOpen = true
                        }}
                    />
                </mdui-tooltip>
            </div>
            <div class="flex px-2 gap-16 justify-end items-center">
                <div class="max-w-md">
                    <SearchBar bind:value={searchQuery} />
                </div>
                <div class="flex">
                    <mdui-segmented-button-group
                        selects="single"
                        value={sortBy}
                        onchange={handleSortChange}
                    >
                        <mdui-segmented-button value="title"
                            >标题</mdui-segmented-button
                        >
                        <mdui-segmented-button value="artist"
                            >歌手</mdui-segmented-button
                        >
                        <mdui-segmented-button value="album"
                            >专辑</mdui-segmented-button
                        >
                    </mdui-segmented-button-group>
                </div>
            </div>
        </div>

        {#if isBatchMode}
            <div
                class="mt-4 flex flex-wrap items-center justify-between gap-3 rounded-2xl bg-[rgb(var(--mdui-color-surface-container-high))] px-4 py-3"
            >
                <div
                    class="text-sm text-[rgb(var(--mdui-color-on-surface-variant))]"
                >
                    已选择 {selectedTrackIds.length} 首歌曲
                </div>

                <div class="flex flex-wrap items-center gap-2">
                    <Button
                        variant="tonal"
                        icon="playlist_add--rounded"
                        disabled={selectedTrackIds.length === 0}
                        onclick={addSelectedToPlaylist}
                    >
                        添加到播放列表
                    </Button>

                    <Button
                        variant="tonal"
                        icon="delete_outline--rounded"
                        disabled={selectedTrackIds.length === 0}
                        onclick={removeSelectedFromCurrentList}
                    >
                        从当前列表移除
                    </Button>

                    <Button
                        variant="tonal"
                        icon="delete--rounded"
                        disabled={selectedTrackIds.length === 0}
                        onclick={() => {
                            isDeleteConfirmDialogOpen = true
                        }}
                    >
                        移至回收站
                    </Button>

                    <Button
                        variant="text"
                        icon="delete_forever--rounded"
                        disabled={selectedTrackIds.length === 0}
                        onclick={() => {
                            isDeleteConfirmDialogOpen = true
                        }}
                        class="bg-[rgb(var(--mdui-color-error-container))] text-[rgb(var(--mdui-color-on-error-container))] [[disabled]]:bg-[rgba(var(--mdui-color-on-surface))]/12 [[disabled]]:text-[rgba(var(--mdui-color-on-surface))]/38"
                    >
                        从磁盘删除
                    </Button>

                    <Button
                        variant="text"
                        icon="arrow_back--rounded"
                        onclick={closeBatchMode}
                    >
                        返回/退出
                    </Button>
                </div>
            </div>
        {/if}
    </header>

    {#if musicLibrary.isLoading}
        <div class="flex flex-1 items-center justify-center">
            <mdui-circular-progress></mdui-circular-progress>
        </div>
    {:else if musicLibrary.error}
        <div class="flex flex-1 items-center justify-center text-red-500">
            {musicLibrary.error}
        </div>
    {:else if displayTracks.length === 0}
        <div
            class="flex flex-1 flex-col items-center justify-center gap-2 text-[rgb(var(--mdui-color-on-surface-variant))]"
        >
            <div class="text-5xl">🎵</div>
            <div class="text-base font-medium">没有找到歌曲</div>
            <div class="text-sm">尝试刷新媒体库或修改搜索关键词</div>
        </div>
    {:else}
        <div class="min-h-0 flex-1 overflow-auto">
            <TrackList
                tracks={displayTracks as Track[]}
                selectable={isBatchMode}
                selectedIds={selectedTrackIds}
                ontoggletrack={toggleTrackSelection}
                onplaytrack={playContextTrack}
                oninsertnexttrack={insertContextTrackNext}
                onshowtrackfolder={showContextTrackFolder}
                onremovetrackfromlist={removeContextTrackFromList}
                ondeletetrackfile={deleteContextTrackFile}
            />
        </div>
    {/if}
</div>

<mdui-dialog
    open={isLibraryDialogOpen}
    close-on-overlay-click
    close-on-esc
    onclose={() => {
        isLibraryDialogOpen = false
    }}
>
    <div
        class="grid max-h-[min(480px,calc(100vh-6rem))] w-100 grid-rows-[auto_auto_minmax(0,1fr)_auto] gap-6 overflow-hidden p-1"
    >
        <div class="w-full">
            <div
                class="text-xl font-semibold text-[rgb(var(--mdui-color-on-surface))]"
            >
                管理媒体文件夹
            </div>
            <div
                class="mt-1 text-sm text-[rgb(var(--mdui-color-on-surface-variant))]"
            >
                配置音乐媒体库扫描目录和启动行为
            </div>
        </div>

        <SettingsRow title="启动时扫描" description="应用启动时自动刷新媒体库">
            <mdui-switch
                checked={settings.data?.scanOnStartup}
                onchange={(event: Event) =>
                    handleSwitchChange(event, 'scanOnStartup')}
            ></mdui-switch>
        </SettingsRow>

        <div class="grid min-h-0 gap-3">
            <div>
                <div class="text-sm font-medium">媒体库目录</div>
                <div
                    class="text-xs text-[rgb(var(--mdui-color-on-surface-variant))]"
                >
                    管理应用扫描音乐文件的本地文件夹
                </div>
            </div>

            <div class="flex">
                <Button variant="tonal" onclick={scanDirectory}>
                    添加本地文件夹
                </Button>
            </div>

            {#if settings.data?.libraryDirs?.length > 0}
                <div class="grid max-h-72 gap-2 overflow-y-auto pr-1 mt-2">
                    {#each settings.data.libraryDirs as dir (dir)}
                        <SettingsListItem title={dir}>
                            <Button
                                variant="text"
                                onclick={() => removeMusicDirectory(dir)}
                            >
                                <span class="text-red-500">移除</span>
                            </Button>
                        </SettingsListItem>
                    {/each}
                </div>
            {:else}
                <div
                    class="rounded-2xl border border-dashed border-[rgb(var(--mdui-color-outline-variant))] p-4 text-sm text-[rgb(var(--mdui-color-on-surface-variant))]"
                >
                    暂未添加媒体库目录，应用将无法扫描到任何歌曲。
                </div>
            {/if}
        </div>

        <div class="flex justify-end">
            <Button
                variant="tonal"
                onclick={() => {
                    isLibraryDialogOpen = false
                }}
            >
                关闭
            </Button>
        </div>
    </div>
</mdui-dialog>

<mdui-dialog
    open={isDeleteConfirmDialogOpen}
    close-on-overlay-click
    close-on-esc
    onclose={() => {
        isDeleteConfirmDialogOpen = false
    }}
>
    <div class="grid w-[min(480px,calc(100vw-3rem))] gap-5 p-1">
        <div>
            <div
                class="text-xl font-semibold text-[rgb(var(--mdui-color-on-surface))]"
            >
                确认删除所选歌曲
            </div>
            <div
                class="mt-2 text-sm leading-6 text-[rgb(var(--mdui-color-on-surface-variant))]"
            >
                将对已选择的 {selectedTrackIds.length} 首歌曲执行删除操作。移至回收站后通常可以恢复；从磁盘中删除将永久移除文件且无法撤销。
            </div>
        </div>

        <div class="flex flex-wrap justify-end gap-2">
            <Button
                variant="text"
                onclick={() => {
                    isDeleteConfirmDialogOpen = false
                }}
            >
                取消
            </Button>

            <Button
                variant="filled"
                icon="delete--rounded"
                onclick={moveSelectedToTrash}
            >
                移至回收站
            </Button>

            <Button
                variant="filled"
                icon="delete_forever--rounded"
                class="bg-[rgb(var(--mdui-color-error-container))] text-[rgb(var(--mdui-color-on-error-container))] [[disabled]]:bg-[rgba(var(--mdui-color-on-surface))]/12 [[disabled]]:text-[rgba(var(--mdui-color-on-surface))]/38"
                onclick={deleteSelectedFromDisk}
            >
                从磁盘中删除
            </Button>
        </div>
    </div>
</mdui-dialog>

<mdui-snackbar
    open={isAddAllSnackbarOpen}
    auto-close-delay="1000"
    onclose={() => {
        isAddAllSnackbarOpen = false
    }}
>
    成功添加至播放列表
</mdui-snackbar>
