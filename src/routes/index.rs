use crate::db::get_library;
use actix_web::web;
use sqlx::{Pool, Sqlite};

pub async fn index(
    state: web::Data<crate::state::AppState>,
    db: web::Data<Pool<Sqlite>>,
) -> super::GenResponse {
    let conn = db.acquire().await?;
    let songs = get_library(conn).await?;
    let mut context = tera::Context::new();
    context.insert("songs", &songs);
    state.render_template("library.j2", &mut context)
}
