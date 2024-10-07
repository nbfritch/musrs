import { useAppDispatch } from '../app/hooks';
import { playSong } from '../features/player/playerSlice';
import { ISong } from '../types/songs';

export interface ISongProps {
  index: number;
  playing?: boolean;
  song: ISong;
  playlistId: "library" | number;
}

export const Song = (props: ISongProps) => {
  const dispatch = useAppDispatch();
  const { song: s, playlistId } = props;
  return (
    <div id={`song-${s.id}`} class={`divTableRow ${s.is_present ? '' : 'missingSong'}`} onClick={() => dispatch(playSong({ playlistId, song: s, playlistIndex: props.index }))}>
      <div class="divTableCell col-id">{props.playing ? 'P' : ''}</div>
      <div class="divTableCell col-track-name">{`${s.track_name} ${s.is_present == false ? '[MISSING]' : ''}`}</div>
      <div class="divTableCell col-dur">{s.duration}</div>
      <div class="divTableCell col-artist">{s.artist}</div>
      <div class="divTableCell col-album">{s.album}</div>
      <div class="divTableCell col-track-num">{s.track_number}</div>
      <div class="divTableCell col-genre">{s.genre}</div>
      <div class="divTableCell col-year">{s.release_year}</div>
    </div>
  );
};
