use crate::types::PartialSong;
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
