use crate::db::get_library;
use actix_web::{
    web::{self},
    HttpResponse,
};
use serde_json::json;
use sqlx::{Pool, Sqlite};

pub async fn get_songs(db: web::Data<Pool<Sqlite>>) -> super::GenResponse {
    let conn = db.acquire().await?;
    let songs = get_library(conn).await?;
    Ok(HttpResponse::Ok().json(json!({ "songs": songs})))
}
