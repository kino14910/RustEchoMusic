import type { AppSettings } from '$lib/types'
import { invoke } from '@tauri-apps/api/core'

export const DEFAULT_SETTINGS: AppSettings = {
    theme: 'system',
    volume: 80,
    libraryDirs: [],
    scanOnStartup: false,
    reduceMotion: false,
}

class SettingsState {
    data = $state<AppSettings>({ ...DEFAULT_SETTINGS })
    isLoading = $state(false)
    error = $state<string | null>(null)

    #saveTimer: ReturnType<typeof setTimeout> | null = null

    async load(): Promise<AppSettings> {
        this.isLoading = true
        this.error = null

        try {
            this.data = await invoke<AppSettings>('load_settings')
            return this.data
        } catch (error) {
            console.error('加载设置失败:', error)
            this.error = String(error)
            return this.data
        } finally {
            this.isLoading = false
        }
    }

    update(patch: Partial<AppSettings>) {
        this.data = {
            ...this.data,
            ...patch,
        }

        this.scheduleSave()
    }

    async save(): Promise<AppSettings> {
        this.error = null

        try {
            this.data = await invoke<AppSettings>('save_settings', {
                settings: this.data,
            })

            return this.data
        } catch (error) {
            console.error('保存设置失败:', error)
            this.error = String(error)
            return this.data
        }
    }

    scheduleSave() {
        if (this.#saveTimer) {
            clearTimeout(this.#saveTimer)
        }

        this.#saveTimer = setTimeout(() => {
            void this.save()
        }, 300)
    }

    addLibraryDir(dir: string) {
        const value = dir.trim()

        if (!value) return
        if (this.data.libraryDirs.includes(value)) return

        this.update({
            libraryDirs: [...this.data.libraryDirs, value],
        })
    }

    removeLibraryDir(dir: string) {
        this.update({
            libraryDirs: this.data.libraryDirs.filter((item) => item !== dir),
        })
    }
}

export const settings = new SettingsState()