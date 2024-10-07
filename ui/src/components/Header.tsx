import { useAppDispatch, useAppSelector } from "../app/hooks";
import { getPlayingSong, getPlayingState, goToNextPlaylistTrack, setPlayingState } from "../features/player/playerSlice";
import { ISong } from "../types/songs";

const NowPlaying = ({ song }: { song: ISong | undefined }) => {
  if (song == null) {
    return (<div id="nothing-playing" class="now-playing-info">
      Nothing playing
    </div>)
  }
  return (
    <div id="now-playing-section">
      <div class="bold now-playing-info">
        Now playing:
      </div>
      <div id="playing-track-name" class="now-playing-info">
        {song.track_name}
      </div>
      <div id="playing-track-artist" class="now-playing-info">
        {song.artist}
      </div>
      <div id="playing-track-album" class="now-playing-info">
        {song.album}
      </div>
    </div>
  )
};

export const Header = () => {
  const playingSong = useAppSelector(getPlayingSong);
  const dispatch = useAppDispatch();
  const playingState = useAppSelector(getPlayingState);
  return (
    <header class="navbar">
      <nav>
        <NowPlaying song={playingSong} />
        <div>
          <audio
            id="audio-player"
            controls
            src={playingSong == null ? undefined : `/song/${playingSong.id}`}
            onCanPlay={() => dispatch(setPlayingState(true))}
            onEnded={() => dispatch(goToNextPlaylistTrack())}
            onPlay={() => dispatch(setPlayingState(true))}
            onPause={() => dispatch(setPlayingState(false))}
            preload="auto"
            autoPlay={playingState}>
          </audio>
        </div>
      </nav>
    </header>
  );
};
