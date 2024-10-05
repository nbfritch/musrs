import { createAppSlice } from "../../app/createAppSlice";
import { IGetLibraryResponse } from "../../types/apiResponse";
import { ISong } from "../../types/songs";

export interface PlayerStateSlice {
  player: {
    playing: boolean;
    playingSongIndex?: number;
    playingSong?: ISong;
  };
  library: {
    libraryPlaylist?: Array<number>;
    songs?: Array<ISong>;
    loadingState: "not-started" | "loading" | "done" | "error";
    errorMessage?: string;
  };
}

const initialState: PlayerStateSlice = {
  player: {
    playing: false,
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
    loadLibrary: create.asyncThunk(async () => {
      const resp = await (await fetch('http://localhost:3000/api/songs')).json() as IGetLibraryResponse;
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
    })
  }),
  selectors: {
    getLibraryState: state => state.library,
    getPlayerState: state => state.player
  },
});

export const { getLibraryState, getPlayerState } = playerSlice.selectors;
export const { togglePlaying, loadLibrary } = playerSlice.actions
