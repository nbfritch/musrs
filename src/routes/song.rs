use actix_web::{get, head, http, web, HttpResponse, HttpResponseBuilder, Responder};
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
    let song_path_buf = std::path::Path::new("/privatefiles").join(song.full_path);
    let song_path = song_path_buf.to_string_lossy();
    Ok(HttpResponseBuilder::new(http::StatusCode::OK)
        .insert_header((
            "X-Accel-Redirect",
            http::header::HeaderValue::from_str(&song_path).unwrap(),
        ))
        .finish())
}
