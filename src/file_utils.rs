use sqlx::pool::PoolConnection;
use sqlx::{Pool, Sqlite};

use crate::types::{PartialSong, Song, TrackMetadata};
use std::fs;
use std::io::Result;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Settings {
    pub allowed_extensions: Vec<String>,
}

fn parse_path(allowed_extensions: &[String], rel_path: &Path) -> Option<PartialSong> {
    let ext = rel_path.extension();
    if let Some(extension) = ext {
        let parsed_extension = String::from(extension.to_str().unwrap());
        if allowed_extensions
            .iter()
            .any(|f| *f == *parsed_extension)
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
                full_path: rel_path.to_str().unwrap().to_string(),
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
                let sub_entries = crawl_dir(allowed_extensions, base_path, &full_path)?;
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

pub fn pretty_duration(duration: i64) -> String {
    format!("{}:{:02}", duration / 60, duration % 60)
}

fn unix_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

async fn save_metadata(
    conn: &mut PoolConnection<Sqlite>,
    song: &Song,
    id: i64,
    base_path: &Path,
) -> anyhow::Result<()> {
    let joined_path = base_path.join(song.full_path.clone());
    let abs_path = joined_path.as_path();
    let tag_res = audiotags::Tag::new().read_from_path(abs_path);

    let metadata: TrackMetadata = match tag_res {
        Ok(tag) => TrackMetadata {
            file_artifact_id: id,
            title: tag.title().map(String::from),
            album: tag.album_title().map(String::from),
            artist: tag.artist().map(String::from),
            year: tag.year().map(|y| y as u16),
            duration: tag.duration().map(|d| d.ceil() as u32),
            genre: tag.genre().map(String::from),
            composer: tag.composer().map(String::from),
            track_number: tag.track_number(),
        },
        Err(_e) => {
            TrackMetadata {
                file_artifact_id: id,
                ..Default::default()
            }
        }
    };

    let meta_insert = sqlx::query!(
        "
        insert into track_metadata (
            filesystem_artifact_id,
            artist,
            album,
            track_name,
            genre,
            composer,
            release_year,
            track_number,
            duration
        ) values (
            ?, ?, ?, ?, ?, ?, ?, ?, ? )",
        metadata.file_artifact_id,
        metadata.artist,
        metadata.album,
        metadata.title,
        metadata.genre,
        metadata.composer,
        metadata.year,
        metadata.track_number,
        metadata.duration
    )
    .execute(conn.as_mut())
    .await?;

    println!("Inserted {} rows", meta_insert.rows_affected());

    Ok(())
}

async fn find_or_create_song(conn: &mut PoolConnection<Sqlite>, song: &Song) -> sqlx::Result<i64> {
    let existing_id = sqlx::query!(
        "
        select 
            f.id
        from filesystem_artifacts f
        where
            f.file_name = ?
            and f.file_extension = ?
            and f.relative_path = ?",
        song.file_name,
        song.file_extension,
        song.file_path
    )
    .fetch_optional(conn.as_mut())
    .await
    .map(|r| r.map(|g| g.id))?;

    if let Some(id) = existing_id {
        return Ok(id);
    }

    let now = unix_timestamp();
    let created_id = sqlx::query!(
        "
        insert into filesystem_artifacts (
            relative_path,
            file_name,
            file_extension,
            is_present,
            first_path_segment,
            second_path_segment,
            created_at,
            updated_at
        ) values (
            ?, ?, ?, TRUE, ?, ?, ?, NULL
        ) returning id;",
        song.file_path,
        song.file_name,
        song.file_extension,
        song.artist,
        song.album,
        now,
    )
    .fetch_one(conn.as_mut())
    .await?
    .id;

    Ok(created_id)
}

pub async fn scan_for_unadded(
    base_path: &Path,
    files: &[Song],
    db: &Pool<Sqlite>,
) -> anyhow::Result<()> {
    // for each song
    // look for a song in the same file path
    // if it exists do nothing
    // if it does not exist, create a row
    // For each row in the database,
    // if the file exists in the list we were given
    // do nothing
    // if the file does not exist in the list we were given
    // update the db row to is_present = false
    let mut conn = db.acquire().await?;

    for song in files.iter() {
        let song_id = find_or_create_song(&mut conn, song).await?;
        let has_meta = sqlx::query!(
            "
            select filesystem_artifact_id from track_metadata
            where filesystem_artifact_id = ?
        ",
            song_id
        )
        .fetch_optional(conn.as_mut())
        .await?
        .is_some();
        if !has_meta {
            save_metadata(&mut conn, song, song_id, base_path).await?;
        }
    }

    Ok(())
}

pub async fn scan_and_flag_missing(base_path: &Path, db: &Pool<Sqlite>,) -> anyhow::Result<()> {
    let mut conn = db.acquire().await?;
    let songs = sqlx::query!("
        select
            id,
            relative_path,
            is_present
        from filesystem_artifacts
    ").fetch_all(conn.as_mut()).await?.iter().map(|r| (r.id, r.relative_path.clone(), r.is_present)).collect::<Vec::<_>>();
    for song in songs.iter() {
        let song_path = base_path.join(song.1.clone());
        let song_exists = fs::exists(song_path).unwrap_or(false);
        if !song_exists {
            let update_res = sqlx::query!("
                update filesystem_artifacts
                set is_present = 0
                where id = ?
            ", song.0).execute(conn.as_mut()).await?;
            if update_res.rows_affected() == 1 {
                println!("Missing: {}", song.1);
            }
        }
    }
    Ok(())
}
