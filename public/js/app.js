const getPlayer = () => document.getElementById('audio-player');

const displayNothingPlaying = (state) => {
    const message = document.getElementById('nothing-playing');
    const nowPlaying = document.getElementById('now-playing-section');
    if (message == null || nowPlaying == null) {
        return;
    }
    if (state) {
        message.classList.remove("hidden");
        nowPlaying.classList.add("hidden");
    } else {
        message.classList.add("hidden");
        nowPlaying.classList.remove("hidden");
    }
};

const displayTrackMetadata = (songId) => {
    const artist = document.getElementById(`song-${songId}-artist`);
    const npArtist = document.getElementById("playing-track-artist");
    if (artist == null || npArtist == null) {
        return;
    }
    npArtist.innerHTML = artist.innerHTML;

    const album = document.getElementById(`song-${songId}-album`);
    const npAlbum = document.getElementById("playing-track-album");
    if (album == null || npAlbum == null) {
        return;
    }
    npAlbum.innerHTML = album.innerHTML;

    const trackName = document.getElementById(`song-${songId}-name`);
    const npTrackName = document.getElementById("playing-track-name");
    if (trackName == null || npTrackName == null) {
        return;
    }
    npTrackName.innerHTML = trackName.innerHTML;
};

const setCurrentSongId = (songId) => {
    const id = document.getElementById("playing-song-id");
    if (id != null) {
        id.innerHTML = `${songId}`;
    }
};

const setCurrentPlaylistId = (playlistId) => {
    const current = document.getElementById("playing-playlist-id");
    if (current != null) {
        current.innerHTML = `${playlistId}`;
    }
};

const selectSongIndex = (idx) => {
    const audioElement = getPlayer();
    const library = document.getElementById('library-body');
    if (library == null) {
        return;
    }
    const totalLibrarySongs = library.children.length;
    const song = library.children[idx > totalLibrarySongs ? idx % totalLibrarySongs : idx];
    if (song == null) {
        return null;
    }
    const songIdStr = song.getAttribute('data-songid');
    if (songIdStr == null) {
        return;
    }

    const songId = parseInt(songIdStr, 10);
    audioElement.setAttribute('src', `/song/${songId}`);
    audioElement.setAttribute('data-songid', songId);
    audioElement.setAttribute('data-playlistid', idx);
    displayNothingPlaying(false);
    setCurrentSongId(songId);
    displayTrackMetadata(songId);
    audioElement.play();
}

const changeSongIdx = (offset) => {
    const audioElement = getPlayer();
    const currentPlaylistId = parseInt(audioElement.getAttribute('data-playlistid'), 10);
    const nextPlaylistId = currentPlaylistId + offset;
    selectSongIndex(nextPlaylistId);
    setCurrentPlaylistId(currentPlaylistId + offset);
};

const nextSong = () => {
    changeSongIdx(1);
};

const prevSong = () => {
    changeSongIdx(-1);
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
        const playlistId = tableEl.getAttribute('data-playlistid');
        if (playlistId == null) {
            return;
        }
        tableEl.addEventListener('click', (e) => {
            e.preventDefault();
            selectSongIndex(playlistId);
        });
    });
};

document.addEventListener('DOMContentLoaded', _e => main());
