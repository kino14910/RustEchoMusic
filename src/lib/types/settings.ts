
export type ThemeMode = 'system' | 'light' | 'dark'

export interface AppSettings {
    theme: ThemeMode
    volume: number
    libraryDirs: string[]
    scanOnStartup: boolean
    reduceMotion: boolean
}