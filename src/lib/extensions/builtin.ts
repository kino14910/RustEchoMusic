import { extensionRegistry } from './registry.svelte'

let registered = false

export function registerBuiltinExtensions() {
    if (registered) return

    registered = true

    extensionRegistry.register({
        id: 'kino.library',
        name: 'Library',
        description: '媒体库基础功能',
        navItems: [
            {
                id: 'tracks',
                title: '歌曲',
                icon: 'music_note--rounded',
                href: '/tracks',
                order: 10,
            },
            {
                id: 'artists',
                title: '歌手',
                icon: 'person--rounded',
                href: '/artists',
                order: 20,
            },
            {
                id: 'albums',
                title: '专辑',
                icon: 'album--rounded',
                href: '/albums',
                order: 30,
            },
            {
                id: 'recent',
                title: '最近播放',
                icon: 'history--rounded',
                href: '/recent',
                order: 40,
            },
        ],
    })

    extensionRegistry.register({
        id: 'kino.settings',
        name: 'Settings',
        description: '应用设置页面',
        navItems: [
            {
                id: 'settings',
                title: '设置',
                icon: 'settings--rounded',
                href: '/settings',
                order: 100,
            },
        ],
    })
}