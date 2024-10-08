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

#[derive(Default)]
pub struct TrackMetadata {
    pub file_artifact_id: i64,
    pub title: Option<String>,
    pub album: Option<String>,
    pub artist: Option<String>,
    pub year: Option<u16>,
    pub duration: Option<u32>,
    pub genre: Option<String>,
    pub composer: Option<String>,
    pub track_number: Option<u16>,
}

#[derive(Serialize)]
pub struct LibraryRow {
    pub id: i64,
    pub track_name: String,
    pub duration: Option<String>,
    pub artist: String,
    pub album: String,
    pub track_number: Option<u16>,
    pub genre: Option<String>,
    pub composer: Option<String>,
    pub release_year: Option<u16>,
    pub is_present: bool,
}
