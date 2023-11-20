const getPlayer = () => document.getElementById('audio-player');

const selectSong = (songId) => {
    const audioElement = getPlayer();
    audioElement.setAttribute('src', `/song/${songId}`);
    audioElement.setAttribute('data-songid', songId);
    audioElement.play();
};

const changeSong = (offset) => {
    const audioElement = getPlayer();
    const currentSongId = parseInt(audioElement.getAttribute('data-songid'), 10);
    audioElement.setAttribute('src', `/song/${currentSongId + offset}`);
    audioElement.setAttribute('data-songid', currentSongId + offset);
    audioElement.play();
}

const nextSong = () => {
    changeSong(1);
};

const prevSong = () => {
    changeSong(-1);
};

const togglePlay = () => {
    const player = getPlayer();
    if (player.paused) {
        player.play();
    } else {
        player.pause();
    }
};

const main = () => {
    const playerEl = getPlayer();
    playerEl.addEventListener('ended', () => nextSong());

    document.addEventListener('keydown', ev => {
        let keep = false;
        console.log(ev.code);
        switch (ev.code) {
            case 'KeyN':
                keep = true;
                nextSong();
                break;
            case 'KeyD':
                keep = true;
                prevSong();
                break;
            case 'Space':
                keep = true;
                togglePlay();
                break;
        }

        if (keep) {
            ev.preventDefault();
        }
    });

    const allRows = Array.from(document.getElementsByClassName('divTableRow'));
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