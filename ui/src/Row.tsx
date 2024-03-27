import { Setter } from "solid-js";

export interface ITrack {
  id: number;
  name: string;
  duration: string;
  artist: string;
  album: string;
  trackNumber: number;
  genre: string;
  releaseYear: string;
}

export interface IRowProps {
  index: number;
  track: ITrack;
  isPlaying: boolean;
  handleClick: Setter<ITrack | null>
}

const Row = (props: IRowProps) => {
  const { index, track, isPlaying, handleClick } = props;
  return (    
    <div class={`table-row hover:text-blue-600 ${ index % 2 === 0 ? 'bg-blue-200' : 'bg-white' }`} onClick={() => handleClick(track)}>
      <div class="table-cell w-[2%]">{isPlaying ? 'P' : ''}</div>
      <div class="table-cell w-[32%]">{track.name}</div>
      <div class="table-cell w-[2%]">{track.duration}</div>
      <div class="table-cell w-[22%] pl-10">{track.artist}</div>
      <div class="table-cell w-[19%]">{track.album}</div>
      <div class="table-cell w-[3%]">{track.trackNumber}</div>
      <div class="table-cell w-[10%]">{track.genre}</div>
      <div class="table-cell w-[8%]">{track.releaseYear}</div>
    </div>
  );
};

export default Row;
