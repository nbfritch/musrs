use actix_web::{web, HttpRequest, HttpResponse, Responder, get};
use serde_json::json;
use std::fs::File;
use std::io::{Read, Result, Seek, SeekFrom};

use crate::types::Song;

const MAX_RANGE_BYTES: u64 = 256_000u64;

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
    let mut f = File::open(file_path).unwrap();
    let file_length = f.metadata().unwrap().len();
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
    songs: web::Data<Vec<Song>>,
    path: web::Path<u64>
) -> impl Responder {
    let song_id = path.into_inner();
    let range_str = request.headers().get("Range").unwrap().to_str().unwrap();
    let (start_byte, maybe_end_byte) = parse_range(range_str);
    let (file_path, content_type) = {
        let song_values = songs.into_inner();
        if song_id as usize >= song_values.len() {
            return HttpResponse::BadRequest().json(json!({"message": "Invalid song id"}));
        }
        let song = song_values[song_id as usize].clone();
        let content_type = extension_content_type(song.file_extension);
        (song.file_path, content_type)
    };
    let base_path = state.into_inner().library_path.clone();
    let absolute_path = std::path::Path::new(&base_path).join(file_path).to_str().unwrap().to_string();
    let (astart, _aend, le, buf) = serve_file_byte_range(absolute_path, start_byte, maybe_end_byte).unwrap();
    
    let resp = HttpResponse::PartialContent()
        .append_header((
            "Content-Range",
            format!("{} {}-{}/{}", "bytes", astart, astart + buf.len() as u64, le),
        ))
        .append_header(("Content-Type", content_type))
        .body(buf);
    resp
}