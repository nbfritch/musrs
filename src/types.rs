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
            filename: self.filename.clone(),
            filepath: self.filepath.clone(),
            full_path: self.full_path.clone(),
            extension: self.extension.clone(),
            artist: self.artist.clone(),
            album: self.album.clone(),
        }
    }
}

#[derive(Clone, Serialize)]
pub struct Song {
    pub id: u64,
    pub filename: String,
    pub filepath: String,
    pub extension: String,
    pub artist: String,
    pub album: String,
    pub full_path: String,
}