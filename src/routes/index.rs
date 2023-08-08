use actix_web::web;

use crate::types::Song;

pub async fn index(
    state: web::Data<crate::state::AppState>,
    songs: web::Data<Vec<Song>>,
) -> super::GenResponse {
    let mut context = tera::Context::new();
    // context.insert("current_readings", &current_temps_result.unwrap());
    context.insert("songs", &songs);
    state.render_template("library.j2", &mut context)
}
