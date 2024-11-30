import { useAppDispatch } from '../app/hooks';
import { setVolume } from '../features/player/playerSlice';
import volumeDown from '../svg/volume-down.svg';
import volumeUp from '../svg/volume-up.svg';

export default function VolumeSlider() {
  const dispatch = useAppDispatch();
  return (
    <div class="py-6 h-7 flex flex-row">
      <span class="basis-1/12"><img src={volumeDown} /></span>
      <input
        type="range"
        min="0"
        max="100"
        onChange={(e) => {
          console.log(e.currentTarget.value);
          dispatch(setVolume(parseInt(e.currentTarget.value) / 100));
        }}
        class="w-30 h-2 basis-10/12 bg-slate-500 appearance-none outline-none cursor-pointer mt-2 mx-2" />
      <span class="basis-1/12"><img src={volumeUp} /></span>
    </div>
  );
}
