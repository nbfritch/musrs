pub type AppState = std::sync::Arc<AppStateStruct>;

pub struct AppStateStruct {
    pub library_path: String,
}

impl AppStateStruct {
    pub fn new(library_path: String) -> Self {
        Self {
            library_path,
        }
    }
}
