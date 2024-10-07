use crate::file_utils::pretty_duration;
use crate::types::LibraryRow;
use sqlx::{pool::PoolConnection, Sqlite};

pub async fn get_library(mut conn: PoolConnection<Sqlite>) -> Result<Vec<LibraryRow>, sqlx::Error> {
    Ok(sqlx::query!(
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
            t.track_number,
            f.is_present
        from filesystem_artifacts f
        left join track_metadata t
            on t.filesystem_artifact_id = f.id
        ) a
        order by lower(a.artist), lower(a.album), a.track_number, a.track_name
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
        is_present: r.is_present != 0,
    })
    .collect::<Vec<_>>())
}
