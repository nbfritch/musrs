use actix_files::NamedFile;
use actix_web::{get, head, web, HttpRequest, HttpResponse, Responder};
use sqlx::{Pool, Sqlite};

use crate::types::Song;

#[head("/song/{song_id}")]
async fn song_head() -> super::GenResponse {
    let resp = HttpResponse::Ok()
        .append_header(("Accept-Ranges", "bytes"))
        .finish();
    Ok(resp)
}

#[get("/song/{song_id}")]
async fn get_song(
    request: HttpRequest,
    state: web::Data<crate::state::AppState>,
    db: web::Data<Pool<Sqlite>>,
    path: web::Path<u64>,
) -> Result<impl Responder, crate::errors::GenError> {
    let song_id = path.into_inner() as i64;
    let mut conn = db.acquire().await?;
    let song = sqlx::query!(
        "
        select
            id,
            relative_path,
            file_name,
            file_extension,
            first_path_segment,
            second_path_segment
        from filesystem_artifacts
        where id = ?",
        song_id
    )
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
        return Err(crate::errors::GenError::Other(
            "song not found in db".into(),
        ));
    }
    let song = song.unwrap();
    let base_path = state.into_inner().library_path.clone();
    let absolute_path = std::path::Path::new(&base_path).join(song.full_path);
    let f = NamedFile::open(absolute_path)?;

    Ok(f.into_response(&request))
}
