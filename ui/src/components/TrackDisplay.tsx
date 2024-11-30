import { useAppDispatch, useAppSelector } from "../app/hooks";
import { getPlayerState } from "../features/player/playerSlice";
import { ISong } from "../types/songs";

export interface ITrackDisplayProps {
  song?: ISong;
  songPlaybackPercentage?: number;
}

export default function TrackDisplay(props: ITrackDisplayProps) {
  if (props.song == null) {
    return (
      <div class="p-1 my-2 mx-8 bg-yellow-100 h-15 rounded-xl leading-4 text-xs border border-solid border-slate-300">
        <p>Nothing Playing</p>
      </div>
    )
  }
  // const dispatch = useAppDispatch();
  const state = useAppSelector(getPlayerState);

  return (
    <div class="p-1 my-2 mx-8 bg-yellow-100 h-16 rounded-xl leading-4 text-xs border border-solid border-slate-300">
      <p>{props.song.track_name}</p>
      <p>{props.song.artist} - {props.song.album}</p>
      <span>
        <span>0:00</span>
        <span class="mx-3 pt-2">
          <input
            min="0"
            max={state.trackDuration}
            value={state.trackPosition}
            type="range"
            class="pt-2 h-1 slider w-[80%]" />
        </span>
        <span>-{props.song.duration}</span>
      </span>
    </div>
  );
}