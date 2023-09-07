mod errors;
mod file_utils;
mod routes;
mod state;
mod types;

use std::env::var;
use std::path::Path;

use actix_web::{HttpServer, middleware::Logger, App, web};
use actix_web_static_files::ResourceFiles;
use file_utils::{crawl_dir, Settings};
use sqlx::sqlite::SqlitePoolOptions;
use types::Song;

use crate::{state::AppStateStruct, routes::index::index, routes::song::get_song, file_utils::dump_tags};

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to load env");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let lib_path = var("MUS_DIR").expect("MUS_DIR var is required");
    let web_port_str: String = var("WEB_PORT").expect("WEB_PORT var is required");
    let web_port: u16 = web_port_str.parse().expect("Could not parse web port");
    let web_addr_string = var("WEB_ADDR").expect("WEB_ADDR var is required");
    let web_addr = web_addr_string.as_str();
    let start_path = Path::new(&lib_path);
    println!("Loading library...");
    let songs: Vec<Song> = tokio::task::block_in_place(|| {
        let extns = vec!["ogg", "flac", "mp3", "wav"];
        let settings = Settings {
            allowed_extensions: extns.iter().map(|e| (**e).to_string()).collect(),
        };
        let mut songs = crawl_dir(&settings.allowed_extensions, start_path, start_path).unwrap();
        songs.sort_unstable_by_key(|a| (a.artist.clone(), a.album.clone(), a.filename.clone()));
        songs.iter().enumerate().map(|ps| ps.1.with_id(ps.0 as u64)).collect()
    });
    println!("Done loading library. Loaded {} songs", songs.len());

    let mut max_path_len: usize = 0;
    let mut max_filename_len: usize = 0;
    for s in songs.iter() {
        if s.file_path.len() > max_path_len {
            max_path_len = s.file_path.len();
        }

        if s.file_name.len() > max_filename_len {
            max_filename_len = s.file_name.len();
        }
    }

    println!("Max Path {}", max_path_len);
    println!("Max File {}", max_filename_len);

    // dump_tags(start_path, &songs);

    let template_folder = Path::new("./templates");

    let db_url = var("DATABASE_URL").expect("'DATABASE_URL is required");
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Could not connect to db");

    HttpServer::new(move || {
        let generated = generate();
        let song_clone = songs.clone();
        let state = std::sync::Arc::new(AppStateStruct::new({
            let mut tera = tera::Tera::new(
                &(template_folder
                    .to_str()
                    .expect("Cannot load templates folder")
                    .to_string()
                    + "/**/*"),
            )
            .expect("Paring error loading templates folder");
            tera.autoescape_on(vec!["j2"]);
            tera
        }, lib_path.clone()));

        App::new()
            .wrap(Logger::default())
            .service(ResourceFiles::new("/static", generated))
            .service(web::resource("/").to(index))
            .service(get_song)
            .app_data(web::Data::new(state))
            .app_data(web::Data::new(song_clone))
            .app_data(pool.clone())
    })
    .bind((web_addr, web_port))
    .expect("Could not bind address")
    .run()
    .await
    .expect("Could not start server");
}
