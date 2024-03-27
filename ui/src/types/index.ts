export interface ITrack {
    id: number;
    name: string;
    duration: string;
    artist: string;
    album: string;
    trackNumber: number;
    genre: string;
    releaseYear: string;
}

export interface IRowProps {
    index: number;
    track: ITrack;
    isPlaying: boolean;
}

export interface ILibraryState {
    currentlyPlaying?: ITrack;
}