import { useEffect } from "preact/hooks";
import { useAppDispatch, useAppSelector } from "../../app/hooks";
import { getLibraryState, loadLibrary } from "./playerSlice";
import { Song } from "../../components/Song";
import { Header } from "../../components/Header";

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
      <Header />
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
          {songs.map((song, index) => <Song index={index} playing={false} song={song} playlistId={"library"} />)}
        </div>
      </div>
    </>
  )
};
