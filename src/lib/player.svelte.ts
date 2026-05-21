type PlayerState = {
  current: Track | null
  isPlaying: boolean
  playlist: Track[]
  progress: number
  duration: number
  volume: number
  isMuted: boolean
  previousVolume: number
}

type Track = {
  title: string
  artist: string
  path: string
}

export const playerState = $state<PlayerState>({
  current: null,
  isPlaying: false,
  playlist: [],
  progress: 0,
  duration: 0,
  volume: 0.8,
  isMuted: false,
  previousVolume: 0.8,
})

export interface SongInfo {
  title: string;
  artist: string;
  album: string;
  duration: number;
  sample_rate?: number;
  bit_depth?: number;
  channels?: number;
}