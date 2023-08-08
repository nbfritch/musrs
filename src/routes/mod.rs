pub mod index;
pub mod song;

pub type GenResponse = Result<actix_web::HttpResponse, crate::errors::GenError>;
