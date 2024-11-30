import Playlist from './Playlist';

export default function PlaylistPane() {
  return (
    <div class="p-0 m-0 w-full h-full bg-gray-50">
      <Playlist listId={1} playlistName={'Cool Jams'} isSelected={false} />
      <Playlist listId={2} playlistName={'Mint Jams'} isSelected={false} />
      <Playlist listId={3} playlistName={'Sweet Jams'} isSelected={true} />
      <Playlist listId={4} playlistName={'Fun Jams'} isSelected={false} />
    </div>
  );
}
