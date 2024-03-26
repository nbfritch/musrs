import './App.css';
import Row from './Row';

function App() {
  return (
    <>
      <header class="navbar top-0 w-full bg-white pb-4 fixed h-[76px]">
        <nav>
          <div id="nothing-playing" class="pl-20 hidden">
            Nothing playing
          </div>
          <div id="now-playing-section" class="flex">
            <div class="font-bold pl-20">
                Now playing:
            </div>
            <div id="playing-track-id" class="hidden">1</div>
            <div id="playing-track-name" class="now-playing-info">
            </div>
            <div id="playing-track-artist" class="now-playing-info">
            </div>
            <div id="playing-track-album" class="now-playing-info">
            </div>
          </div>
          <div>
            <audio id="audio-player" class="pt-5 px-20 w-[90%]" controls preload="auto" data-playlist-id="0"></audio>
          </div>
        </nav>
      </header>

      <div class="table w-full" style="border: 1px solid #000;">
        <div class="table-header-group bg-gray-200 top-76 w-full font-bold fixed h-[20px]">
          <div class="table-row">
            <div class="table-cell w-[2%]"></div>
            <div class="table-cell w-[32%]">Track Name</div>
            <div class="table-cell w-[2%]">Time</div>
            <div class="table-cell w-[22%] pl-10">Artist</div>
            <div class="table-cell w-[19%]">Album</div>
            <div class="table-cell w-[3%]">#</div>
            <div class="table-cell w-[10%]">Genre</div>
            <div class="table-cell w-[8%]">Year</div>
          </div>
        </div>
        <div class="w-full pt-76 table mt-20">
          <Row />
        </div>
      </div>
    </>
  );
}

export default App
