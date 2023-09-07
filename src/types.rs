use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct PartialSong {
    pub filename: String,
    pub filepath: String,
    pub extension: String,
    pub artist: String,
    pub album: String,
    pub full_path: String,
}

impl PartialSong {
    pub fn with_id(&self, id: u64) -> Song {
        Song {
            id,
            file_name: self.filename.clone(),
            file_path: self.filepath.clone(),
            full_path: self.full_path.clone(),
            file_extension: self.extension.clone(),
            artist: self.artist.clone(),
            album: self.album.clone(),
        }
    }
}

#[derive(Clone, Serialize)]
pub struct Song {
    pub id: u64,
    pub file_name: String,
    pub file_path: String,
    pub file_extension: String,
    pub artist: String,
    pub album: String,
    pub full_path: String,
}

pub struct TrackMetadata {
    title: String,
    album: Option<String>,
    artist: Option<String>,
    year: Option<u16>,
    duration: Option<u32>,
    genre: Option<String>,
    composer: Option<String>,
}
