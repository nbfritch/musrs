import { useEffect } from "preact/hooks";
import { useAppDispatch, useAppSelector } from "../../app/hooks";
import { getLibraryState, loadLibrary } from "./playerSlice";
import { Song } from "../../components/Song";

export const Player = () => {
  const dispatch = useAppDispatch();
  const libraryState = useAppSelector(getLibraryState);
  useEffect(() => {
    dispatch(loadLibrary())
  }, []);

  if (libraryState.loadingState === "loading") {
    return (
      <p>Loading...</p>
    );
  }

  if (libraryState.loadingState === "error" || libraryState.songs == null) {
    return (
      <>
        <h1>Error</h1>
        <p>{libraryState.errorMessage}</p>
      </>
    );
  }

  const { songs } = libraryState;

  return (
    <>
    <header class="navbar">
      <nav>
        <div id="nothing-playing" class="now-playing-info">
          Nothing playing
        </div>
        <div id="now-playing-section" class="hidden">
          <div class="bold now-playing-info">
            Now playing:
          </div>
          <div id="playing-track-id" class="hidden">1</div>
          <div id="playing-playlist-id" class="hidden">0</div>
          <div id="playing-track-name" class="now-playing-info">
          </div>
          <div id="playing-track-artist" class="now-playing-info">
          </div>
          <div id="playing-track-album" class="now-playing-info">
          </div>
        </div>
        <div>
          <audio id="audio-player" controls src="http://localhost:3000/song/1" preload="auto"></audio>
        </div>
      </nav>
    </header>
    <div class="divTable" style="border: 1px solid #000;">
      <div class="divTableHeading">
        <div class="divTableRow">
          <div class="divTableHead col-id"></div>
          <div class="divTableHead col-track-name">Track Name</div>
          <div class="divTableHead col-dur">Time</div>
          <div class="divTableHead col-artist">Artist</div>
          <div class="divTableHead col-album">Album</div>
          <div class="divTableHead col-track-num">#</div>
          <div class="divTableHead col-genre">Genre</div>
          <div class="divTableHead col-year">Year</div>
        </div>
      </div>
      <div id="library-body" class="divTableBody">
        {songs.map((song, index) => <Song index={index} playing={false} song={song} />)}
      </div>
    </div>
    </>
  )
};
