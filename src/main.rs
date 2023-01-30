use actix_files::NamedFile;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::fs::File;
use std::io::{Read, Result, Seek, SeekFrom};

const MAX_RANGE_BYTES: u64 = 64_000u64;

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
    start_byte: u64,
    end_byte: Option<u64>,
) -> Result<(u64, u64, u64, Vec<u8>)> {
    let mut f = File::open("static/song1.flac").unwrap();
    let file_length = f.metadata().unwrap().len();
    let use_end_byte = match end_byte {
        Some(b) => b,
        None => start_byte + MAX_RANGE_BYTES,
    };
    if use_end_byte - start_byte > MAX_RANGE_BYTES {
        f.seek(SeekFrom::Start(start_byte))?;
        let mut buf = vec![0; MAX_RANGE_BYTES as usize];
        f.read_exact(&mut buf)?;
        Ok((start_byte, use_end_byte, file_length, buf))
    } else {
        f.seek(SeekFrom::Start(start_byte))?;
        let mut buf = vec![0; (use_end_byte - start_byte) as usize];
        f.read_exact(&mut buf)?;
        Ok((start_byte, use_end_byte, file_length, buf))
    }
}

async fn index(_req: HttpRequest) -> Result<NamedFile> {
    Ok(NamedFile::open("static/index.html")?)
}

async fn song(request: HttpRequest) -> impl Responder {
    let range_requested = request.headers().contains_key("Range");

    let range_str = request.headers().get("Range").unwrap().to_str().unwrap();
    let (start_byte, maybe_end_byte) = parse_range(range_str);
    let (astart, aend, le, buf) = serve_file_byte_range(start_byte, maybe_end_byte).unwrap();
    let resp = HttpResponse::PartialContent()
        .append_header((
            "Content-Range",
            format!("{} {}-{}/{}", "bytes", astart, aend, le),
        ))
        .append_header(("Content-Type", "audio/flac"))
        .body(buf);
    resp
}

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/song", web::get().to(song))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
