
export type ThemeMode = 'auto' | 'light' | 'dark'

export interface AppSettings {
    theme: ThemeMode
    volume: number
    libraryDirs: string[]
    scanOnStartup: boolean
    reduceMotion: boolean
}