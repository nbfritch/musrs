import playIcon from '../svg/play.svg';
import fastForwardIcon from '../svg/step-forward.svg';
import rewindIcon from '../svg/step-backward.svg';
import pauseIcon from '../svg/pause.svg';
import { useAppDispatch, useAppSelector } from '../app/hooks';
import { getPlayerState, setPlayingState } from '../features/player/playerSlice';

export default function PlaybackControls() {
  const playerState = useAppSelector(getPlayerState);
  const dispatch = useAppDispatch();
  return (
    <div class="py-4">
        <button class="mx-2 h-10 w-10"><img src={rewindIcon} /></button>
        <button
          class="mx-2 h-10 w-10"
          onClick={() => { dispatch(setPlayingState(!playerState.playing)) }}
          >{playerState.playing ? <img src={pauseIcon} /> : <img src={playIcon} />}</button>
        <button class="mx-2 h-10 w-10"><img src={fastForwardIcon} /></button>
    </div>
  );
};