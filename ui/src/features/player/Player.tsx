import { useEffect } from "preact/hooks";
import { useAppDispatch, useAppSelector } from "../../app/hooks";
import { getLibraryState, getPlayingSong, getPlayingState, loadLibrary, playSong } from "./playerSlice";
import { Header } from "../../components/Header";
import PlaylistPane from "../../components/PlaylistPane";
import { ISong } from "../../types/songs";
import playIcon from "../../svg/play.svg";

export interface ISongProps {
  index: number;
  playing?: boolean;
  song: ISong;
  playlistId: "library" | number;
}

const Song = (props: ISongProps) => {
  const dispatch = useAppDispatch();
  const { song: s, playlistId } = props;
  return (
    <tr id={`song-${s.id}`} class={`${s.is_present ? '' : 'missingSong'}`} onClick={() => dispatch(playSong({ playlistId, song: s, playlistIndex: props.index }))}>
      <td class="divTableCell col-id">{props.playing ? <img class='h-4' src={playIcon} /> : <></>}</td>
      <td class="divTableCell col-track-name">{`${s.track_name} ${s.is_present == false ? '[MISSING]' : ''}`}</td>
      <td class="divTableCell col-dur">{s.duration}</td>
      <td class="divTableCell col-artist">{s.artist}</td>
      <td class="divTableCell col-album">{s.album}</td>
      <td class="divTableCell col-track-num">{s.track_number}</td>
      <td class="divTableCell col-genre">{s.genre}</td>
      <td class="divTableCell col-year">{s.release_year}</td>
    </tr>
  );
};

export const Player = () => {
  const dispatch = useAppDispatch();
  const playingSong = useAppSelector(getPlayingSong);
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
      <div id="browser-root" class="mt-20 min-w-200 resize resize-x h-full w-full flex">
        <div class="w-[20%] border-right border-solid border-r-3 border-slate-400 flex flex-row">
          <PlaylistPane />
          <div class="w-2 bg-slate-400 h-full cursor-move"></div>
        </div>
        <div id="pane-root" class="min-w-200 resize resize-x h-full w-full flex">
          <table class="w-full">
            <thead class="text-left">
              <tr>
                <th scope="col">id</th>
                <th scope="col">Track Name</th>
                <th scope="col">Time</th>
                <th scope="col">Artist</th>
                <th scope="col">Album</th>
                <th scope="col">#</th>
                <th scope="col">Genre</th>
                <th scope="col">Year</th>
              </tr>
            </thead>
            <tbody>
              {songs.map((song, index) => <Song index={index} playing={song.id == playingSong?.id} song={song} playlistId={"library"} />)}
            </tbody>
          </table>
        </div>
      </div>
    </>
  )
};
