export interface Track {
    title: string
    artist: string
    album: string
    albumArtist: string
    duration: number
    sampleRate?: number | null
    cover?: string | null
    path: string
}

export interface Playlist {
  id: number
  name: string
  cover: string
}


export interface Artist {
    id: string
    name: string
    cover?: string | null
    trackCount: number
    albumCount: number
    tracks: Track[]
}


export interface Album {
    id: string
    title: string
    cover?: string | null
    trackCount: number
    tracks: Track[]
    representativeTrack: Track
}