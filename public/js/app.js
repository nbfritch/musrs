const getPlayer = () => document.getElementById('audio-player');

const selectSong = (songId) => {
    const audioElement = getPlayer();
    audioElement.setAttribute('src', `/song/${songId}`);
    audioElement.setAttribute('data-songid', songId);
    audioElement.play();
};

const nextSong = () => {
    const audioElement = getPlayer();
    const currentSongId = parseInt(audioElement.getAttribute('data-songid'));
    audioElement.setAttribute('src', `/song/${currentSongId + 1}`);
    audioElement.setAttribute('data-songid', currentSongId + 1);
    audioElement.play();
};

const main = () => {
    const playerEl = getPlayer();
    playerEl.addEventListener('ended', () => nextSong());

    const allRows = Array.from(document.getElementsByClassName('table-row'));
    allRows.forEach(tableEl => {
        const songId = tableEl.getAttribute('data-songid');
        if (songId == null) {
            return;
        }
        tableEl.addEventListener('click', (e) => {
            e.preventDefault();
            selectSong(songId);
        });
    });
};

document.addEventListener('DOMContentLoaded', _e => main());