#[derive(Debug)]
pub enum GenError {
    OtherError(String),
    TemplateError(tera::Error),
}

impl actix_web::error::ResponseError for GenError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            Self::OtherError(_) => actix_web::http::StatusCode::SEE_OTHER,
            Self::TemplateError(_) => actix_web::http::StatusCode::SERVICE_UNAVAILABLE,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        match self {
            GenError::OtherError(_) => {
                actix_web::HttpResponse::build(actix_web::http::StatusCode::SEE_OTHER)
                    .insert_header((
                        actix_web::http::header::LOCATION,
                        actix_web::http::header::HeaderValue::from_static("error"),
                    ))
                    .finish()
            }
            GenError::TemplateError(_) => {
                actix_web::HttpResponse::build(actix_web::http::StatusCode::SERVICE_UNAVAILABLE)
                    .body("<h1>Please, try again later</h1>")
            }
        }
    }
}

impl std::fmt::Display for GenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenError::OtherError(e) => write!(f, "other error: {e}"),
            GenError::TemplateError(e) => write!(f, "cannot parse template: {e}"),
        }
    }
}

impl From<String> for GenError {
    fn from(value: String) -> Self {
        Self::OtherError(value)
    }
}

impl From<tera::Error> for GenError {
    fn from(value: tera::Error) -> Self {
        Self::TemplateError(value)
    }
}