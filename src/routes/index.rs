use actix_web::web;
use sqlx::{Pool, Sqlite};

use crate::{file_utils::pretty_duration, types::LibraryRow};

pub async fn index(
    state: web::Data<crate::state::AppState>,
    db: web::Data<Pool<Sqlite>>,
) -> super::GenResponse {
    let mut conn = db.acquire().await?;
    let songs = sqlx::query!(
        "
        select * from (
        select
            f.id,
            f.file_name,
            f.first_path_segment,
            f.second_path_segment,
            ifnull(t.artist, f.first_path_segment) as artist,
            ifnull(t.album, f.second_path_segment) as album,
            ifnull(t.track_name, f.file_name) as track_name,
            t.genre,
            t.composer,
            t.release_year,
            t.duration,
            t.track_number
        from filesystem_artifacts f
        left join track_metadata t
            on t.filesystem_artifact_id = f.id
        ) a
        order by a.artist, a.album, a.track_number, a.track_name
    "
    )
    .fetch_all(conn.as_mut())
    .await?
    .iter()
    .map(|r| LibraryRow {
        id: r.id,
        track_name: r.track_name.clone(),
        duration: r.duration.map(pretty_duration),
        artist: r
            .artist
            .clone()
            .or(r.first_path_segment.clone())
            .unwrap_or(String::from("Unknown")),
        album: r
            .album
            .clone()
            .or(r.second_path_segment.clone())
            .unwrap_or(String::from("Unknown")),
        track_number: r.track_number.map(|t| t as u16),
        genre: r.genre.clone(),
        composer: r.composer.clone(),
        release_year: r.release_year.map(|t| t as u16),
    })
    .collect::<Vec<_>>();

    let mut context = tera::Context::new();
    context.insert("songs", &songs);
    state.render_template("library.j2", &mut context)
}
