import { PayloadAction } from "@reduxjs/toolkit";
import { createAppSlice } from "../../app/createAppSlice";
import { IGetLibraryResponse } from "../../types/apiResponse";
import { ISong } from "../../types/songs";
import { getUrl } from "../../utils/url";

export interface PlayerStateSlice {
  player: {
    playing: boolean;
    playingPlaylistId: "library" | number;
    playingSongIndex?: number;
    playingSong?: ISong;
    volume?: number;
    trackDuration?: number;
    trackPosition?: number;
  };
  library: {
    libraryPlaylist?: Array<number>;
    songs?: Array<ISong>;
    loadingState: "not-started" | "loading" | "done" | "error";
    errorMessage?: string;
  };
}

export interface IPlaySongAction {
  song: ISong;
  playlistIndex: number;
  playlistId: "library" | number;
}

const initialState: PlayerStateSlice = {
  player: {
    playing: false,
    playingPlaylistId: "library"
  },
  library: {
    loadingState: "not-started",
  },
};

export const playerSlice = createAppSlice({
  name: "player",
  initialState,
  reducers: create => ({
    togglePlaying: create.reducer(state => {
      state.player.playing = !state.player.playing;
    }),
    setPlayingState: create.reducer((state, action: PayloadAction<boolean>) => {
      state.player.playing = action.payload;
    }),
    setVolume: create.reducer((state, action: PayloadAction<number>) => {
      state.player.volume = action.payload;
    }),
    setTrackDuration: create.reducer((state, action: PayloadAction<number>) => {
      state.player.trackDuration = action.payload;
    }),
    setTrackPlaybackPosition: create.reducer((state, action: PayloadAction<number>) => {
      state.player.trackPosition = action.payload;
    }),
    goToNextPlaylistTrack: create.reducer(state => {
      if (state.library.libraryPlaylist == null || state.library.libraryPlaylist.length == 0 || state.library.songs == null) {
        // No music available to play
        return;
      }

      // TODO: This should check the current playing playlist
      if (state.player.playingSongIndex == state.library.libraryPlaylist.length - 1) {
        // End of playlist
        state.player.playing = false;
        state.player.playingSong = undefined;
        state.player.playingSongIndex = undefined;
      }

      if (state.player.playingSongIndex == null) {
        // We weren't playing before
        state.player.playingSongIndex = 0;
      }

      let maybeNextIndex = state.player.playingSongIndex + 1;
      let nextSongId = state.library.libraryPlaylist[maybeNextIndex];
      let maybeNextSong = state.library.songs.find(song => song.id === nextSongId);
      while (maybeNextIndex < state.library.songs.length && maybeNextSong?.is_present === false) {
        maybeNextIndex = maybeNextIndex + 1;
        nextSongId = state.library.libraryPlaylist[maybeNextIndex];
        maybeNextSong = state.library.songs.find(song => song.id === nextSongId);
      }

      if (maybeNextSong != null) {
        state.player.playingSong = maybeNextSong;
        state.player.playingSongIndex = maybeNextIndex;
      }
    }),
    loadLibrary: create.asyncThunk(async () => {
      const resp = await (await fetch(getUrl('/api/songs'))).json() as IGetLibraryResponse;
      return resp;
    }, {
      pending: state => {
        state.library.loadingState = "loading";
      },
      fulfilled: (state, action) => {
        state.library.libraryPlaylist = action.payload.songs.map(song => song.id);
        state.library.songs = action.payload.songs;
        state.library.loadingState = "done";
      },
      rejected: state => {
        state.library.loadingState = "error";
        state.library.errorMessage = "Failed to load library";
      }
    }),
    playSong: create.reducer((state, action: PayloadAction<IPlaySongAction>) => {
      state.player.playingPlaylistId = action.payload.playlistId;
      state.player.playingSongIndex = action.payload.playlistIndex;
      state.player.playingSong = action.payload.song;
      state.player.playing = true;
    }),
  }),
  selectors: {
    getLibraryState: state => state.library,
    getPlayerState: state => state.player,
    getPlayingSong: state => state.player.playingSong,
    getPlayingState: state => state.player.playing,
  },
});

export const { getLibraryState, getPlayerState, getPlayingSong, getPlayingState } = playerSlice.selectors;
export const { togglePlaying, loadLibrary, playSong, setPlayingState, goToNextPlaylistTrack, setTrackDuration, setTrackPlaybackPosition, setVolume } = playerSlice.actions
