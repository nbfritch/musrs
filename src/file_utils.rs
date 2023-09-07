use sqlx::{Pool, Sqlite};

use crate::types::{PartialSong, Song};
use std::fs;
use std::io::Result;
use std::path::Path;

pub struct Settings {
    pub allowed_extensions: Vec<String>,
}

fn parse_path(allowed_extensions: &Vec<String>, rel_path: &Path) -> Option<PartialSong> {
    let ext = rel_path.extension();
    if let Some(extension) = ext {
        let parsed_extension = String::from(extension.to_str().unwrap());
        if allowed_extensions
            .iter()
            .find(|f| **f == *parsed_extension)
            .is_some()
        {
            let filepath = String::from(rel_path.to_str().unwrap());
            let segments = filepath.split("/").collect::<Vec<_>>();
            let (artist, album) = if segments.len() == 3 {
                (segments[0], segments[1])
            } else {
                ("Unknown", "Unknown")
            };
            let filename_with_ext = String::from(rel_path.file_name().unwrap().to_str().unwrap());
            let ext = String::from(extension.to_str().unwrap());
            let filename = filename_with_ext.replace(&format!(".{}", ext), "");
            let l = PartialSong {
                filename,
                filepath: filepath.clone(),
                extension: String::from(extension.to_str().unwrap()),
                artist: String::from(artist),
                album: String::from(album),
                full_path: rel_path.to_str().unwrap().to_string()
            };
            return Some(l);
        }
    }
    None
}

pub fn crawl_dir(
    allowed_extensions: &Vec<String>,
    base_path: &Path,
    dir: &Path,
) -> Result<Vec<PartialSong>> {
    let mut entries: Vec<PartialSong> = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            print!(".");
            let entry = entry?;
            let full_path = entry.path();
            if full_path.is_dir() {
                let sub_entries = crawl_dir(allowed_extensions, &base_path, &full_path)?;
                for sentry in sub_entries {
                    entries.push(sentry);
                }
            } else {
                let full_path = entry.path();
                let rel_path = full_path
                    .strip_prefix(base_path)
                    .expect("Could not strip prefix of file");
                let p = parse_path(allowed_extensions, rel_path);
                if let Some(part_song) = p {
                    entries.push(part_song);
                }
            }
        }
    }

    Ok(entries)
}

fn pretty_duration(duration: f64) -> String {
    let int_duration = duration.ceil() as u64;
    format!("{}:{:02}", int_duration / 60, int_duration % 60)
}

pub async fn startup_scan(base_path: &Path, files: &Vec<Song>, db: &Pool<Sqlite>) {
    // for each song
    // look for a song in the same file path
    // if it exists do nothing
    // if it does not exist, create a row
    // For each row in the database,
    // if the file exists in the list we were given
    // do nothing
    // if the file does not exist in the list we were given
    // update the db row to is_present = false
    let mut conn = db.acquire().await.expect("Could not aquire db handle");

    for song in files.iter() {
        let has_existing_song = sqlx::query!("
            select f.id from filesystem_artifacts f
            where f.file_name = ? and f.file_extension = ? and f.relative_path = ?
            ", song.file_name, song.file_extension, song.file_path)
            .fetch_optional(conn.as_mut())
            .await
            .map(|r| {
                r.map(|r| r.id > 0)
            }).unwrap_or(Some(false))
            .unwrap_or(false);
    }
}

pub fn dump_tags(base_path: &Path, files: &Vec<Song>) {
    files.iter().for_each(|song| {
        let joined_path = base_path.join(song.full_path.clone());
        let abs_path = joined_path.as_path();
        let tag = audiotags::Tag::new().read_from_path(abs_path);
        match tag {
            Ok(btag) => {
                println!("Read tags for {}", song.full_path);
                btag.title().map(|t| println!("Title: {}", t));
                btag.artists().map(|a| a.iter().map(|art| println!("Artist(s): {}", *art)).collect::<Vec<()>>());
                btag.year().map(|y| println!("Year: {}", y));
                btag.duration().map(|d| pretty_duration(d)).map(|d| println!("Duration: {}", d));
                btag.album_title().map(|at| println!("Album Title: {}", at));
                btag.album_artists().map(|aa| aa.iter().map(|a| println!("Album Artist: {}", *a)).collect::<Vec<()>>());
                btag.track_number().map(|t| println!("Track Number: {}", t));
                btag.total_tracks().map(|t| println!("Total Tracks: {}", t));
                btag.disc_number().map(|d| println!("Disc: {}", d));
                btag.total_discs().map(|td| println!("Total Discs: {}", td));
                btag.genre().map(|g| println!("Genre: {}", g));
                btag.composer().map(|c| println!("Composer: {}", c));
                println!("");
            }
            Err(e) => {
                println!("Err parsing {}: {}", song.full_path, e);
                println!("");
            },
        }
    })
}