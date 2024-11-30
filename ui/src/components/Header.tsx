import { useEffect, useRef } from "preact/hooks";
import { useAppDispatch, useAppSelector } from "../app/hooks";
import { getPlayerState, getPlayingSong, getPlayingState, goToNextPlaylistTrack, setPlayingState, setTrackDuration, setTrackPlaybackPosition } from "../features/player/playerSlice";
import { ISong } from "../types/songs";
import { getUrl } from "../utils/url";
import PlaybackControls from "./Controls";
import GlobalSearch from "./GlobalSearch";
import TrackDisplay from "./TrackDisplay";
import VolumeSlider from "./VolumeSlider";

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
  const playerState = useAppSelector(getPlayerState);
  const audioElementRef = useRef<HTMLAudioElement>(null);

  useEffect(() => {
    if (playerState.playing === false) {
      audioElementRef?.current?.pause();
    } else {
      audioElementRef?.current?.play();
    }
  }, [playerState.playing]);

  return (
    <header class="w-full h-20 fixed top-0 overflow-hidden bg-slate-200 flex flex-row border-solid border-1 border-slate-500">
      <div id="playback-controls" class="basis-2/12 text-center">
        <PlaybackControls />
      </div>
      <div id="volume-slider" class="basis-2/12 text-center">
        <VolumeSlider />
      </div>
      <div id="current-song-display" class="basis-4/12 text-center">
        <TrackDisplay song={playingSong} />
      </div>
      <div id="global-search-bar" class="basis-4/12 text-center">
        <GlobalSearch />
      </div>
      <audio
        id="audio-player"
        ref={audioElementRef}
        style="display: none"
        controls
        src={playingSong == null ? undefined : getUrl(`/song/${playingSong.id}`)}
        volume={playerState.volume}
        onCanPlay={() => dispatch(setPlayingState(true))}
        onEnded={() => dispatch(goToNextPlaylistTrack())}
        onPlay={() => dispatch(setPlayingState(true))}
        onPause={() => dispatch(setPlayingState(false))}
        onDurationChange={(e) => {
          dispatch(setTrackDuration(e.currentTarget.duration))
        }}
        onTimeUpdate={(e) => {
          dispatch(setTrackPlaybackPosition(e.currentTarget.currentTime))
        }}
        preload="auto"
        autoPlay={playingState}>
      </audio>
    </header>
  );
};
