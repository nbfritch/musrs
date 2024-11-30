import playlistIcon from '../svg/playlist.svg';

export interface IPlaylistProps {
  playlistName: string;
  listId: number;
  isSelected: boolean;
}

export default function Playlist(props: IPlaylistProps) {
  const bgColor = props.isSelected ? 'bg-blue-200' : props.listId % 2 === 0 ? 'bg-gray-50' : 'bg-gray-100';
  return (
    <div class={`flex pl-3 py-1 ${bgColor}`}>
      <span class="inline-block w-4 h-4 mt-[1px]"><img src={playlistIcon} /></span><div class="pl-2">{props.playlistName}</div>
    </div>
  );
}
