// Tauri doesn't have a Node.js server to do proper SSR
// so we use adapter-static with a fallback to index.html to put the site in SPA mode
// See: https://svelte.dev/docs/kit/single-page-apps
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info

import { musicLibrary } from '$lib/state/library.svelte'
import { recentlyPlayed } from '$lib/state/recent.svelte'
import { settings } from '$lib/state/settings.svelte'

export const prerender = true
export const ssr = false
export async function load() {
    await settings.load()

    await Promise.all([
        recentlyPlayed.load(),
        musicLibrary.load(),
    ])

    return {}
}