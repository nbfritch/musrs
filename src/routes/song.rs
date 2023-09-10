use actix_web::{get, web, HttpRequest, HttpResponse};
use serde_json::json;
use sqlx::{Pool, Sqlite};
use std::fs::File;
use std::io::{Read, Result, Seek, SeekFrom};

use crate::types::Song;

const MAX_RANGE_BYTES: u64 = 1_000_000u64;

fn parse_range(s: &str) -> (u64, Option<u64>) {
    assert_eq!(&s[0..6], "bytes=");
    let split = (&s[6..])
        .split("-")
        .map(|a| a.to_string())
        .collect::<Vec<String>>();
    assert_eq!(split.len(), 2);
    (
        u64::from_str_radix(&split[0], 10).unwrap(),
        match u64::from_str_radix(&split[1], 10) {
            Err(_) => None,
            Ok(a) => Some(a),
        },
    )
}

fn serve_file_byte_range(
    file_path: String,
    start_byte: u64,
    end_byte: Option<u64>,
) -> Result<(u64, u64, u64, Vec<u8>)> {
    let mut f = File::open(file_path)?;
    let file_length = f.metadata()?.len();
    let use_end_byte = match end_byte {
        Some(b) => b,
        None => start_byte + MAX_RANGE_BYTES,
    };
    if use_end_byte - start_byte > MAX_RANGE_BYTES {
        f.seek(SeekFrom::Start(start_byte))?;
        let mut buf = vec![0; MAX_RANGE_BYTES as usize];
        f.read(&mut buf)?;
        Ok((start_byte, use_end_byte, file_length, buf))
    } else {
        f.seek(SeekFrom::Start(start_byte))?;
        let mut buf = vec![0; (use_end_byte - start_byte) as usize];
        f.read(&mut buf)?;
        Ok((start_byte, use_end_byte, file_length, buf))
    }
}

fn extension_content_type(extension: String) -> String {
    let s = if extension == "flac" {
        "audio/flac"
    } else if extension == "mp3" {
        "audio/mpeg3"
    } else if extension == "ogg" {
        "audio/vorbis"
    } else if extension == "wav" {
        "audio/wav"
    } else {
        "audio/mpeg3"
    };

    String::from(s)
}

#[get("/song/{song_id}")]
async fn get_song(
    request: HttpRequest,
    state: web::Data<crate::state::AppState>,
    db: web::Data<Pool<Sqlite>>,
    path: web::Path<u64>,
) -> super::GenResponse {
    let song_id = path.into_inner() as i64;
    let mut conn = db.acquire().await?;
    let song = sqlx::query!("
        select
            id,
            relative_path,
            file_name,
            file_extension,
            first_path_segment,
            second_path_segment
        from filesystem_artifacts
        where id = ?",
        song_id)
        .fetch_optional(conn.as_mut())
        .await?
        .map(|r| Song {
            id: r.id as u64,
            file_name: r.file_name,
            file_path: r.relative_path.clone(),
            file_extension: r.file_extension,
            artist: r.first_path_segment.unwrap_or(String::from("Unknown")),
            album: r.second_path_segment.unwrap_or(String::from("Unknown")),
            full_path: r.relative_path,
        });

    if song.is_none() {
        return Ok(HttpResponse::NotFound().json(json!({"message": "Song with id not found"})));
    }
    let song = song.unwrap();
    let range_str = request.headers().get("Range").unwrap().to_str().unwrap();
    let (start_byte, maybe_end_byte) = parse_range(range_str);
    let content_type = extension_content_type(song.file_extension);
    let base_path = state.into_inner().library_path.clone();
    let absolute_path = std::path::Path::new(&base_path)
        .join(song.full_path)
        .to_str()
        .unwrap()
        .to_string();
    println!("{}", absolute_path);
    let (astart, _aend, le, buf) =
        serve_file_byte_range(absolute_path, start_byte, maybe_end_byte)?;

    let resp = HttpResponse::PartialContent()
        .append_header((
            "Content-Range",
            format!(
                "{} {}-{}/{}",
                "bytes",
                astart,
                astart + buf.len() as u64,
                le
            ),
        ))
        .append_header(("Content-Type", content_type))
        .body(buf);
    Ok(resp)
}
