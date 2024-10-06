use std::io;

#[derive(Debug)]
pub enum GenError {
    Other(String),
    Database(sqlx::Error),
}

impl actix_web::error::ResponseError for GenError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            Self::Other(_) => actix_web::http::StatusCode::SEE_OTHER,
            Self::Database(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        match self {
            GenError::Other(_) => {
                actix_web::HttpResponse::build(actix_web::http::StatusCode::SEE_OTHER)
                    .insert_header((
                        actix_web::http::header::LOCATION,
                        actix_web::http::header::HeaderValue::from_static("error"),
                    ))
                    .finish()
            }
            GenError::Database(e) => {
                actix_web::HttpResponse::build(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
                    .body(format!("<div><h1>Error</h1><p>{}</p></div>", e))
            }
        }
    }
}

impl std::fmt::Display for GenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenError::Other(e) => write!(f, "other error: {e}"),
            GenError::Database(e) => write!(f, "database error: {e}"),
        }
    }
}

impl From<String> for GenError {
    fn from(value: String) -> Self {
        Self::Other(value)
    }
}

impl From<sqlx::Error> for GenError {
    fn from(value: sqlx::Error) -> Self {
        Self::Database(value)
    }
}

impl From<io::Error> for GenError {
    fn from(value: io::Error) -> Self {
        Self::Other(format!("{}", value))
    }
}
