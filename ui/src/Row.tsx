const Row = () => {
  return (    
    <div class="table-row row-{{ loop.index % 2 }}">
      <div class="table-cell w-[2%]"></div>
      <div id="song-{{s.id}}-name" class="table-cell w-[32%]">s.track_name</div>
      <div class="table-cell w-[2%]">s.duration</div>
      <div id="song-{{s.id}}-artist" class="table-cell w-[22%]">s.artist</div>
      <div id="song-{{s.id}}-album" class="table-cell w-[19%]">s.album</div>
      <div class="table-cell w-[3%]">s.track_number</div>
      <div class="table-cell w-[10%]">s.genre</div>
      <div class="table-cell w-[8%]">s.release_year</div>
    </div>
  );
};

export default Row;
