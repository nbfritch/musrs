use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse,
};

use crate::errors;

pub type AppState = std::sync::Arc<AppStateStruct>;

pub struct AppStateStruct {
    templates: tera::Tera,
    pub library_path: String,
}

impl AppStateStruct {
    pub fn new(template: tera::Tera, library_path: String) -> Self {
        Self {
            templates: template,
            library_path,
        }
    }

    pub fn render_template(
        &self,
        template: &str,
        context: &mut tera::Context,
    ) -> Result<HttpResponse, errors::GenError> {
        let current_template = template.replace(".j2", "");
        if !context.contains_key("current_template") {
            context.insert("current_template", &current_template);
        }

        let body = self.templates.render(template, &context)?;
        Ok(HttpResponse::build(StatusCode::OK)
            .content_type(ContentType::html())
            .body(body))
    }
}
